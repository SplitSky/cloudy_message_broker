use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> std::io::Result<()> {
    // bind to locahost
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    println!("Server is listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr()?);

                // read data from client
                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
                        let response = b"Hello client!";
                        stream.write_all(response).expect("Failed to write");
                        stream.flush().expect("Failed to flush");
                    }
                    Err(e) => println!("Error read: {}", e),
                }
            }
            Err(e) => println!("Error accepting: {}", e),
        }
    }
    Ok(())
}
