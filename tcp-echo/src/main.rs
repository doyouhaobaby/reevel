use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incomming connection form : {}", stream.peer_addr()?);

    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    // nc 127.0.0.1 8888
    let listener = TcpListener::bind("0.0.0.0:8888").
        expect("Cound not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {eprintln!("failed: {}", e)},
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
                });
            },
        }
    }
}
