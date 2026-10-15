# Task Sink with Controller

Task sink with a controller socket to send kill signals.

```rust
// examples/zguide/tasksink2/main.rs
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    receiver.bind("tcp://*:5558").unwrap();

    let controller = context.socket(zmq::PUB).unwrap();
    controller.bind("tcp://*:5559").unwrap();

    receiver.recv_bytes(0).unwrap();
    let start = Instant::now();

    for task_nbr in 0..100 {
        receiver.recv_bytes(0).unwrap();
        if task_nbr % 10 == 0 { print!(":"); } else { print!("."); }
        let _ = io::stdout().flush();
    }

    println!("\nTotal elapsed time: {:?}", start.elapsed());
    controller.send("KILL", 0).unwrap();
}
```

## Key Points

- PULL socket receives results from workers
- PUB socket sends control messages (KILL)
- Controller is bound on different port
- After collecting results, sends KILL signal
- Used with taskwork2
