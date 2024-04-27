use crate::cli::GlobalState;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buf = Vec::new();
        // [0; 256];
        println!("reading from stream...");
        stream
            .read_to_end(&mut buf)
            .expect("could not read from stream");

        println!("{:?}", buf);

        stream
            .write("+OK\r\n".as_bytes())
            .expect("could not write to stream");
    }
}

pub fn start_server(state: GlobalState) {
    let listener = TcpListener::bind("0.0.0.0:6379").expect("could not bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => handle_client(tcp_stream),
            Err(ref e) => eprintln!("{:?}", e),
        }
    }
}
