# Multiple Socket Polling

Using zmq::poll() to read from multiple sockets.

```rust
// examples/zguide/mspoller/main.rs
fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    let subscriber = context.socket(zmq::SUB).unwrap();

    receiver.connect("tcp://localhost:5557").unwrap();
    subscriber.connect("tcp://localhost:5556").unwrap();
    subscriber.set_subscribe(b"10001").unwrap();

    let mut msg = zmq::Message::new();
    loop {
        let mut items = [
            receiver.as_poll_item(zmq::POLLIN),
            subscriber.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();

        if items[0].is_readable() && receiver.recv(&mut msg, 0).is_ok() {
            // Process task
        }
        if items[1].is_readable() && subscriber.recv(&mut msg, 0).is_ok() {
            // Process weather update
        }
    }
}
```

## Key Points

- Use zmq::poll() to monitor multiple sockets
- poll() blocks until at least one socket is readable
- Timeout of -1 means infinite wait
- Each socket needs a PollItem created via as_poll_item()
