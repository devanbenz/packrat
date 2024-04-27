use crate::cli::init_state;
use crate::server::start_server;

mod cli;
mod compaction;
mod keyvalue;
mod memtable;
mod server;
mod wal;

fn main() {
    let mut state = init_state();
    start_server(&mut state);
}
