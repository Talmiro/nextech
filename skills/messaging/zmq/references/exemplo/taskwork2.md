# Task Worker with Controller

Task worker that listens to both work and controller sockets.

```rust
// examples/zguide/taskwork2/main.rs
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    let sender = context.socket(zmq::PUSH).unwrap();
    let controller = context.socket(zmq::SUB).unwrap();

    receiver.connect("tcp://localhost:5557").unwrap();
    sender.connect("tcp://localhost:5558").unwrap();
    controller.connect("tcp://localhost:5559").unwrap();
    controller.set_subscribe(b"").unwrap();

    loop {
        let mut items = [
            receiver.as_poll_item(zmq::POLLIN),
            controller.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();

        if items[0].is_readable() {
            let string = receiver.recv_string(0).unwrap().unwrap();
            print!(".");
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_millis(string.parse::<u64>().unwrap()));
            sender.send("", 0).unwrap();
        }
        if items[1].is_readable() {
            break;
        }
    }
}
```

## Key Points

- Uses poll() to listen to two sockets simultaneously
- PULL receives work, SUB receives control messages
- When controller receives KILL, exits loop
- Polls both sockets with -1 (infinite wait)
- Used with tasksink2
