use crate::index::Indexes;
use crate::memtable::MemTable;
use crate::wal::Wal;
use clap::Parser;
use resp::Decoder;
use std::path::PathBuf;

#[derive(Parser)]
pub struct CliArgs {
    pub wal_dir: String,
    pub sstable_dir: String,
    pub sstable_threshold: usize,
}

pub struct GlobalState {
    pub memtable: MemTable,
    pub indexes: Indexes,
    pub sstable_threshold: usize,
}
pub fn init_state() -> GlobalState {
    let cli = CliArgs::parse();
    let wal = Wal::new(PathBuf::from(cli.wal_dir.as_str())).expect("cannot create new WAL");
    let memtable = MemTable::new(wal, PathBuf::from(cli.sstable_dir));
    let indexes = Indexes::new();

    GlobalState {
        memtable,
        indexes,
        sstable_threshold: cli.sstable_threshold,
    }
}
