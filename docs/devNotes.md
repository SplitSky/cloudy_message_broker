# Concepts needed
- TCP
- Sockets
- Binding
- Listening
- Client-Server model

## How does it work under the hood?
1. Socker is made
2. Socker is bound to a specific IP address and port
3. Listening - the server starts to listen for incoming connections
4. Accepting connections - when a client connects it accepts it
5. Reads the data from the client, processes it and sends a response
6. Connections are closed when communication is complete

## Common pitfalls
1. Concurrency -> Handling multiple clients requires proper threading or async/await
2. Resource management -> Close sockets to avoid leaks
3. Robust error handling -> Lean on Rust
4. Implement proper authentication and encryption
5. Optimise for high throughput and low latency
6. Use Rust's modules to organise code

## Crates to consider
### Basic implementation
std::net::{TcpListener, Tcp:Stream}
std::io::{Read, Write}

### Threading
also use std::thread

### Async/await
tokio::net and tokio::io
and also futures::Future

## Testing
use telnet or netcat

Write tests using #[cfg(test)]
