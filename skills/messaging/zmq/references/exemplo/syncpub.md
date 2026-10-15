# Synchronized Publisher

Publisher that waits for subscribers to sync before broadcasting.

```rust
// examples/zguide/syncpub/main.rs
fn main() {
    let context = zmq::Context::new();

    let publisher = context.socket(zmq::PUB).unwrap();
    publisher.set_sndhwm(1_100_000).unwrap();
    publisher.bind("tcp://*:5561").unwrap();

    let syncservice = context.socket(zmq::REP).unwrap();
    syncservice.bind("tcp://*:5562").unwrap();

    println!("Waiting for subscribers");
    for _ in 0..10 {
        syncservice.recv_msg(0).unwrap();
        syncservice.send("", 0).unwrap();
    }

    println!("Broadcasting messages");
    for _ in 0..1_000_000 {
        publisher.send("Rhubarb", 0).unwrap();
    }
    publisher.send("END", 0).unwrap();
}
```

## Key Points

- Publisher binds PUB and REP sockets
- REP socket receives sync requests from subscribers
- Waits for all subscribers to connect before broadcasting
- After sync, broadcasts 1M messages + END
- Prevents slow subscribers from missing initial messages
