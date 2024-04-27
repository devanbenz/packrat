use crate::keyvalue::KeyValue;
use crate::wal::Wal;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
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

    pub fn get(&self, key: String) -> Result<String, MemTableError> {
        if let Some(val) = self.tree.get(&key) {
            Ok(val.to_owned())
        } else {
            Err(MemTableError::GetError)
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<(), MemTableError> {
        if let Some(value) = self.tree.insert(key, value) {
            println!("overwriting {:?}", value);
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn flush_to_sstable(&mut self) {
        println!("flushing to sstable");
        let mut sstable_vec = Vec::new();
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut sstable_file = OpenOptions::new()
            .read(false)
            .write(true)
            .append(true)
            .create(true)
            .open(self.sstable_dir.join(format!("{}_sstable.dat", time_stamp)))
            .unwrap();

        while let Some((key, val)) = self.tree.pop_first() {
            let kv = KeyValue::new((key, val));
            sstable_vec.append(&mut kv.write_to_bytes());
        }

        sstable_file
            .write(&sstable_vec[..])
            .expect("writing to ss_table file");
    }
}

mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_new_memtable_with_wal_data() {
        let wal = Wal::new(PathBuf::from("test_data")).unwrap();
        let memtable = MemTable::new_with_wal_data(wal, PathBuf::from(""));

        assert_eq!(memtable.get("foo".to_string()).unwrap(), "bar".to_string());
        assert_eq!(memtable.get("bar".to_string()).unwrap(), "baz".to_string());
        assert_eq!(
            memtable.get("tadash".to_string()).unwrap(),
            "mizu".to_string()
        );
    }

    #[test]
    fn test_insert_and_get_memtable() {
        let wal = Wal::new(PathBuf::new()).unwrap();
        let mut memtable = MemTable::new(wal, PathBuf::new());

        memtable.set("foo".to_string(), "bar".to_string()).unwrap();
        let val = memtable.get("foo".to_string()).unwrap();

        assert_eq!("bar".to_string(), val);
    }

    #[test]
    fn test_flush_memtable_to_sstabl() {
        let wal = Wal::new(PathBuf::new()).unwrap();
        let mut memtable = MemTable::new(wal, PathBuf::new());

        memtable.set("foo".to_string(), "bar".to_string()).unwrap();
        memtable.set("bar".to_string(), "baz".to_string()).unwrap();
        memtable
            .set("tadashi".to_string(), "mizu".to_string())
            .unwrap();

        memtable.flush_to_sstable();
    }
}
