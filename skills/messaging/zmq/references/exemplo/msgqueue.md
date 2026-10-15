# Simple Message Queue Proxy

Basic ROUTER/DEALER proxy for message queuing.

```rust
// examples/zguide/msgqueue/main.rs
fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::DEALER).unwrap();

    frontend.bind("tcp://*:5559").unwrap();
    backend.bind("tcp://*:5560").unwrap();

    zmq::proxy(&frontend, &backend).expect("failed to proxy");
}
```

## Key Points

- Simplest message queue implementation
- Uses zmq::proxy to connect ROUTER to DEALER
- Clients connect to frontend (ROUTER)
- Workers connect to backend (DEALER)
- Proxy handles message routing automatically
