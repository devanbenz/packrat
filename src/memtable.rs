use crate::cli::GlobalState;
use crate::index::Indexes;
use crate::keyvalue::KeyValue;
use crate::wal::Wal;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::string::FromUtf8Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MemTable {
    pub tree: BTreeMap<String, String>,
    pub sstable_dir: PathBuf,
    pub wal: Wal,
}

#[derive(Debug)]
pub enum MemTableError {
    GetError,
    SetError,
    NoValueFound,
    OtherError(String),
}

impl MemTable {
    pub fn new(wal: Wal, sstable_dir: PathBuf) -> Self {
        Self {
            tree: BTreeMap::new(),
            wal,
            sstable_dir,
        }
    }

    pub fn new_with_wal_data(mut wal: Wal, sstable_dir: PathBuf) -> Self {
        let kv_vec = wal.read();
        let mut tree = BTreeMap::<String, String>::new();

        for kv in kv_vec {
            tree.insert(kv.key, kv.value);
        }

        Self {
            tree,
            wal,
            sstable_dir,
        }
    }

    pub fn get(&self, key: String, indexes: &mut Indexes) -> Result<String, MemTableError> {
        if let Some(val) = self.tree.get(&key) {
            Ok(val.to_owned())
        } else {
            let file_name = match indexes.get_file(&key) {
                Some(v) => v,
                None => return Err(MemTableError::NoValueFound),
            };
            let offset = match indexes.get_offset(&key) {
                Some(v) => v,
                None => return Err(MemTableError::NoValueFound),
            };
            let len = match indexes.get_len(&key) {
                Some(v) => v,
                None => return Err(MemTableError::NoValueFound),
            };

            let mut buf = Vec::with_capacity(len);
            buf.resize(len, 0);

            let mut sstable_file = OpenOptions::new()
                .read(true)
                .write(false)
                .open(self.sstable_dir.join(file_name))
                .unwrap();

            match sstable_file.seek(SeekFrom::Start(offset as u64)) {
                Ok(v) => {
                    println!("seeking {:?}", v);
                }
                Err(e) => {
                    return Err(MemTableError::OtherError(e.to_string()));
                }
            };
            match sstable_file.read_exact(&mut buf[..]) {
                Ok(_) => {
                    println!("successfully read from buffer");
                }
                Err(e) => {
                    return Err(MemTableError::OtherError(e.to_string()));
                }
            };

            match KeyValue::deserialize(buf) {
                Ok(val) => {
                    println!("{:?}", val);
                    Ok(val.get(0).unwrap().1.to_owned())
                }
                Err(e) => {
                    eprintln!("{:?}", e.to_string());
                    return Err(MemTableError::OtherError(e.to_string()));
                }
            }
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), MemTableError> {
        if let Some(value) = self.tree.insert(key.clone(), value.clone()) {
            println!("overwriting {:?}", value);
            self.wal
                .append(KeyValue::new((key, value)))
                .expect("could not append to wal");
            Ok(())
        } else {
            self.wal
                .append(KeyValue::new((key, value)))
                .expect("could not append to wal");
            Ok(())
        }
    }

    pub fn flush_to_sstable(&mut self, indexes: &mut Indexes) {
        println!("flushing to sstable");
        let mut sstable_vec = Vec::new();

        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let sstable_name = format!("{}_sstable.dat", time_stamp);

        let mut sstable_file = OpenOptions::new()
            .read(false)
            .write(true)
            .append(true)
            .create(true)
            .open(self.sstable_dir.join(&sstable_name))
            .unwrap();

        while let Some((key, val)) = self.tree.pop_first() {
            let kv = KeyValue::new((key.clone(), val));
            let kv_as_bytes = &mut kv.write_to_bytes();

            indexes.create_index(
                key,
                sstable_vec.len(),
                kv_as_bytes.len(),
                sstable_name.clone(),
            );
            sstable_vec.append(kv_as_bytes);
        }

        sstable_file
            .write(&sstable_vec[..])
            .expect("writing to ss_table file");

        self.wal.clear_wal();
    }
}

mod tests {
    use super::*;
    use crate::index::Indexes;
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn test_new_memtable_with_wal_data() {
        let tmp_dir = TempDir::new("test_dir").unwrap();
        let file_path = tmp_dir.into_path();
        let tmp_dir_sstable = TempDir::new("test_dir").unwrap();
        let file_path_sstable = tmp_dir_sstable.into_path();

        let wal = Wal::new(file_path).unwrap();
        let mut memtable = MemTable::new(wal, file_path_sstable);
        let mut indexes = Indexes::new();

        memtable.set("foo".to_string(), "bar".to_string()).unwrap();
        memtable.set("bar".to_string(), "baz".to_string()).unwrap();
        memtable
            .set("tadashi".to_string(), "mizu".to_string())
            .unwrap();

        println!("{:?}", memtable.wal.read());
        let new_memtable = MemTable::new_with_wal_data(memtable.wal, memtable.sstable_dir);

        assert_eq!(
            new_memtable.get("foo".to_string(), &mut indexes).unwrap(),
            "bar".to_string()
        );
        assert_eq!(
            new_memtable.get("bar".to_string(), &mut indexes).unwrap(),
            "baz".to_string()
        );
        assert_eq!(
            new_memtable
                .get("tadashi".to_string(), &mut indexes)
                .unwrap(),
            "mizu".to_string()
        );
    }

    #[test]
    fn test_insert_and_get_memtable() {
        let tmp_dir = TempDir::new("test_dir").unwrap();
        let file_path = tmp_dir.into_path();
        let wal = Wal::new(file_path.clone()).unwrap();
        let mut memtable = MemTable::new(wal, file_path);
        let mut indexes = Indexes::new();

        memtable.set("foo".to_string(), "bar".to_string()).unwrap();
        memtable.set("bar".to_string(), "baz".to_string()).unwrap();
        memtable
            .set("tadashi".to_string(), "mizu".to_string())
            .unwrap();

        assert_eq!(
            memtable.get("foo".to_string(), &mut indexes).unwrap(),
            "bar".to_string()
        );
        assert_eq!(
            memtable.get("bar".to_string(), &mut indexes).unwrap(),
            "baz".to_string()
        );
        assert_eq!(
            memtable.get("tadashi".to_string(), &mut indexes).unwrap(),
            "mizu".to_string()
        );
    }

    #[test]
    fn test_flush_memtable_to_sstable() {
        let wal = Wal::new(PathBuf::new()).unwrap();
        let mut memtable = MemTable::new(wal, PathBuf::new());

        memtable.set("foo".to_string(), "bar".to_string()).unwrap();
        memtable.set("bar".to_string(), "baz".to_string()).unwrap();
        memtable
            .set("tadashi".to_string(), "mizu".to_string())
            .unwrap();

        let mut indexes = Indexes::new();
        memtable.flush_to_sstable(&mut indexes);
    }
}
