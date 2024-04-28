use std::collections::BTreeMap;

pub struct Index {
    pub offset: usize,
    pub length: usize,
    pub file_name: String,
}

pub struct Indexes {
    pub indexes: BTreeMap<String, Index>,
}

impl Index {
    pub fn from(offset: usize, length: usize, file_name: String) -> Self {
        Self {
            offset,
            length,
            file_name,
        }
    }
}

impl Indexes {
    pub fn new() -> Self {
        Self {
            indexes: BTreeMap::new(),
        }
    }

    pub fn create_index(&mut self, key: String, offset: usize, length: usize, file_name: String) {
        println!("{:?} {:?} {:?} {:?}", key, length, offset, file_name);
        let index = Index::from(offset, length, file_name);
        match self.indexes.insert(key, index) {
            Some(_) => {
                println!("overwriting existing index");
            }
            None => {
                println!("newly created index");
            }
        }
    }

    pub fn get_file(&self, key: &String) -> Option<String> {
        match self.indexes.get(key) {
            Some(v) => Some(v.file_name.to_string()),
            None => None,
        }
    }

    pub fn get_offset(&self, key: &String) -> Option<usize> {
        match self.indexes.get(key) {
            Some(v) => Some(v.offset),
            None => None,
        }
    }

    pub fn get_len(&self, key: &String) -> Option<usize> {
        match self.indexes.get(key) {
            Some(v) => Some(v.length),
            None => None,
        }
    }
}
