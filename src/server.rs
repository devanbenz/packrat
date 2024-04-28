use crate::cli::GlobalState;
use crate::memtable::MemTableError;
use resp::{Decoder, Value};
use std::io::{BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn process_cmd(state: &mut GlobalState, cmd: &[u8]) -> Result<Vec<u8>, MemTableError> {
    let memtable = &mut state.memtable;
    let indexes = &mut state.indexes;

    let mut resp_parser = Decoder::new(BufReader::new(cmd));
    let val = resp_parser.decode().expect("cannot decode value");

    if let Value::Array(v) = val {
        let mut val_iter = v.iter();
        match val_iter.next() {
            Some(Value::Bulk(v)) => match v.as_str() {
                "get" => {
                    if let Some(key) = val_iter.next() {
                        let value = memtable.get(key.to_string_pretty(), indexes)?;
                        return Ok(format!("+{}\r\n", value.as_str()).into_bytes());
                    } else {
                        return Err(MemTableError::GetError);
                    }
                }
                "set" => {
                    let key = val_iter.next().expect("could not get key");
                    let value = val_iter.next().expect("could not get value");
                    memtable.set(key.to_string_pretty(), value.to_string_pretty())?;
                    if memtable.tree.len() > state.sstable_threshold {
                        memtable.flush_to_sstable(&mut state.indexes);
                    }

                    return Ok(Vec::from("+OK\r\n".as_bytes()));
                }
                "COMMAND" => {
                    return Ok(Vec::from("+OK\r\n".as_bytes()));
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };
    };

    Err(MemTableError::SetError)
}

fn handle_client(mut stream: TcpStream, state: &mut GlobalState) {
    loop {
        let mut buf: [u8; 256] = [0; 256];
        let bytes_read = stream.read(&mut buf).expect("could not read from stream");

        let cmd = &buf[..bytes_read];
        match process_cmd(state, cmd) {
            Ok(ret) => {
                let _ = stream.write(&ret).expect("could not write to stream");
            }
            Err(_) => {
                let _ = stream
                    .write(b"-There was an error\r\n")
                    .expect("could not write to stream");
            }
        }
    }
}

pub fn start_server(state: &mut GlobalState) {
    let listener = TcpListener::bind("0.0.0.0:6379").expect("could not bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => handle_client(tcp_stream, state),
            Err(ref e) => eprintln!("{:?}", e),
        }
    }
}
