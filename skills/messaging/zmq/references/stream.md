# STREAM Pattern

Raw TCP stream - server-side socket.

```rust
let stream = ctx.socket(zmq::STREAM).unwrap();
stream.bind("tcp://*:8080").unwrap();

loop {
    // First frame: client identity
    let id = stream.recv_bytes(0).unwrap();
    // Second frame: actual data
    let data = stream.recv_bytes(0).unwrap();
    
    // Echo back
    stream.send(&id, zmq::SNDMORE).unwrap();
    stream.send(&data, 0).unwrap();
}
```

## Key Points

- Server-side only
- First frame is always identity
- Works like traditional TCP socket
- Use for legacy protocol adapters
- Client uses TCP directly
