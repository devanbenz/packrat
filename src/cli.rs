use crate::memtable::MemTable;
use crate::wal::Wal;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct CliArgs {
    pub wal_dir: String,
    pub sstable_dir: String,
    pub sstable_threshold: u8,
}

pub struct GlobalState {
    pub memtable: MemTable,
    pub sstable_threshold: u8,
}
pub fn init_state() -> GlobalState {
    let cli = CliArgs::parse();
    let wal = Wal::new(PathBuf::from(cli.wal_dir.as_str())).expect("cannot create new WAL");
    let memtable = MemTable::new(wal, PathBuf::from(cli.sstable_dir));

    GlobalState {
        memtable,
        sstable_threshold: cli.sstable_threshold,
    }
}
