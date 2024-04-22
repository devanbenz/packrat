use crate::keyvalue::KeyValue;

pub struct Wal {
    pub log: Vec<Vec<u8>>,
}
