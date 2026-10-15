# Multiple Socket Reader (DONTWAIT)

Reading from multiple sockets using DONTWAIT flag instead of polling.

```rust
// examples/zguide/msreader/main.rs
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    let subscriber = context.socket(zmq::SUB).unwrap();

    receiver.connect("tcp://localhost:5557").unwrap();
    subscriber.connect("tcp://localhost:5556").unwrap();
    subscriber.set_subscribe(b"10001").unwrap();

    let mut msg = zmq::Message::new();
    loop {
        loop {
            if receiver.recv(&mut msg, zmq::DONTWAIT).is_err() { break; }
            // Process task
        }

        loop {
            if subscriber.recv(&mut msg, zmq::DONTWAIT).is_err() { break; }
            // Process weather update
        }

        thread::sleep(Duration::from_millis(1))
    }
}
```

## Key Points

- Uses DONTWAIT flag for non-blocking receives
- Loops until no more messages available on each socket
- Sleeps briefly when no messages to prevent CPU spinning
- Prioritizes traffic by checking first socket fully before second
