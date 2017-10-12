use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        buf = [0; 512];
        let _ = match stream.read(&mut buf) {
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            },
            Err(e) => {
                panic!("{}", e);
            },
        };

        if buf[0] == 13 {
            let _ = stream.write("Bye\n".as_bytes());
            let _ = stream.shutdown(Shutdown::Both);
        }

        match stream.write(&buf) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            },
            Err(e) => {
                println!("{}", e);
            },
        }
    }
}
