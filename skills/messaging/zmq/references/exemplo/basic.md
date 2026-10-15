# Basic Examples

## Version

Get the ZeroMQ library version.

```rust
fn main() {
    let (major, minor, patch) = zmq::version();
    println!("Current 0MQ version is {}.{}.{}", major, minor, patch);
}
```

---

# Socket Types Summary

| Type | Pattern | Direction | Blocking |
|------|---------|-----------|----------|
| PUB | Publisher | Send only | No |
| SUB | Subscriber | Receive only | Yes |
| REQ | Requester | Send/Receive | Yes |
| REP | Replier | Receive/Send | Yes |
| DEALER | Async Requester | Send/Receive | No |
| ROUTER | Async Replier | Receive/Send | Yes |
| PUSH | Producer | Send only | No |
| PULL | Consumer | Receive only | Yes |
| PAIR | Pair | Bidirectional | Yes |
| STREAM | Stream Server | Receive only | Yes |

---

# Common Socket Options

```rust
// Set socket identity (for ROUTER/DEALER)
socket.set_identity(b"worker1").unwrap();

// Set subscription filter (for SUB)
socket.set_subscribe(b"filter").unwrap();

// Set receive timeout (milliseconds)
socket.set_rcvtimeo(1000).unwrap();

// Set send timeout (milliseconds)
socket.set_sndtimeo(1000).unwrap();

// Enable keepalive
socket.set_keepalive(1).unwrap();
```

---

# Transport Protocols

| Protocol | Format | Description |
|----------|--------|-------------|
| TCP | `tcp://host:port` | TCP transport |
| IPC | `ipc://path` | Inter-process communication |
| INPROC | `inproc://name` | In-process (intra-process) |
| PGM | `pgm://address` | Reliable multicast |

---

# Basic Context and Socket

```rust
fn main() {
    // Create ZMQ context
    let context = zmq::Context::new();

    // Create socket
    let socket = context.socket(zmq::PUB).unwrap();

    // Bind to address
    socket.bind("tcp://*:5555").unwrap();

    // Send message
    socket.send("Hello", 0).unwrap();
}
```

## Receiving Messages

```rust
fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SUB).unwrap();
    socket.connect("tcp://localhost:5555").unwrap();
    socket.set_subscribe(b"").unwrap();

    // Blocking receive
    let msg = socket.recv_string(0).unwrap().unwrap();

    // Non-blocking receive (returns None if no message)
    // socket.set_rcvtimeo(0).unwrap();
    // let msg = socket.recv_string(0).unwrap();
}
```

## Multipart Messages

```rust
fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::DEALER).unwrap();

    // Send multipart message
    socket.send("part1", zmq::SNDMORE).unwrap();
    socket.send("part2", 0).unwrap();

    // Receive multipart message
    let parts = socket.recv_multipart(0).unwrap();
    for (i, part) in parts.iter().enumerate() {
        println!("Part {}: {:?}", i, part);
    }
}
```

## Polling

```rust
fn main() {
    let context = zmq::Context::new();
    let socket = context.socket(zmq::SUB).unwrap();
    socket.connect("tcp://localhost:5555").unwrap();

    let mut poll_item = socket.as_poll_item(zmq::POLLIN);
    
    // Wait for 1 second
    zmq::poll(&mut [poll_item], 1000).unwrap();

    if poll_item.is_readable() {
        let msg = socket.recv_string(0).unwrap().unwrap();
        println!("Received: {}", msg);
    }
}
```
