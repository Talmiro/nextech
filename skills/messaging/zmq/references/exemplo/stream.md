# STREAM Socket

STREAM sockets provide raw TCP stream handling.

## Log Server

```rust
use std::str;

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::STREAM).unwrap();
    socket.bind("tcp://*:8888").unwrap();

    loop {
        let data = socket.recv_multipart(0).unwrap();
        println!(
            "Identity: {:?} Message : {}",
            data[0],
            str::from_utf8(&data[1]).unwrap()
        );
    }
}
```

## Key Points

- STREAM is server-side only (use CONNECT)
- First frame is always the client identity
- Second frame is the actual message data
- Useful for handling raw TCP connections
- Similar to traditional socket programming
