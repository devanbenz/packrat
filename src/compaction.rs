// TODO: Need to implement compaction still
// pub trait Compaction {
//     fn compact();
// }

use std::fs;
use std::path::PathBuf;

pub struct CompactionConfig {
    pub levels: usize,
    pub file_limit: usize,
}

impl CompactionConfig {
    pub fn new(levels: usize, file_limit: usize, sstable_dir: PathBuf) -> Self {
        for level in 0..levels {
            fs::create_dir(sstable_dir)
        }
        Self { levels, file_limit }
    }

    pub fn compact(&self, sstable_dir: PathBuf) {
        for level in 0..self.levels {}
    }
}
