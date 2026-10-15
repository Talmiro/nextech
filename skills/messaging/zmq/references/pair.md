# PAIR Pattern

Bidirectional pipe - simple two-way communication.

```rust
// Peer 1
let pair = ctx.socket(zmq::PAIR).unwrap();
pair.bind("inproc://pipe").unwrap();
pair.send("hello", 0).unwrap();
let msg = pair.recv_string(0).unwrap().unwrap();

// Peer 2
let pair = ctx.socket(zmq::PAIR).unwrap();
pair.connect("inproc://pipe").unwrap();
let msg = pair.recv_string(0).unwrap().unwrap();
pair.send("world", 0).unwrap();
```

## Key Points

- Bidirectional, single connection
- No routing or load balancing
- Perfect for thread communication
- Use inproc:// transport
- Cannot connect to multiple peers
