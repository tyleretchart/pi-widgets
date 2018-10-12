use std::io::prelude::*;
use std::io::{self, Read};
use std::net::TcpStream;

pub fn connect(address: &String, data: String) -> String {
    let mut stream = TcpStream::connect(address).expect("Couldn't connect to the server...");
    stream
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let _ = stream.write(data.as_bytes()).unwrap();

    let mut buf = vec![];
    loop {
        match stream.read_to_end(&mut buf) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                // wait_for_fd();
            }
            Err(e) => panic!("encountered IO error: {}", e),
        };
    }
    println!("bytes: {:?}", buf);
    String::from_utf8(buf).expect("Found invalid UTF-8")
}
