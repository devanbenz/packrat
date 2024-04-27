use crate::keyvalue::KeyValue;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Error, Read, Write};
use std::path::PathBuf;

pub struct Wal(File);

impl Wal {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        let wal_file = path.join("wal.dat");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(wal_file)?;

        Ok(Self(file))
    }

    pub fn append(&mut self, key_value: KeyValue) -> Result<(), Error> {
        let serialized_kv_bytes = key_value.write_to_bytes();
        self.0.write(&serialized_kv_bytes[..])?;

        Ok(())
    }

    pub fn read(&mut self) -> Vec<KeyValue> {
        let mut buf = Vec::new();
        let _ = io::BufReader::new(&self.0).read_to_end(&mut buf);

        let kvs = KeyValue::read_from_bytes(buf);
        kvs
    }
}

mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    pub fn test_writing_to_wal() -> Result<(), Error> {
        let tmp_dir = TempDir::new("wal_test")?;
        let file_path = tmp_dir.into_path();
        let mut wal = Wal::new(file_path)?;
        let key_value = KeyValue::new(("foo".to_string(), "bar".to_string()));

        wal.append(key_value)?;
        let kvs = wal.read();
        for i in kvs {
            assert_eq!(
                i,
                KeyValue {
                    key: "foo".to_string(),
                    value: "bar".to_string(),
                }
            );
        }

        Ok(())
    }
}
