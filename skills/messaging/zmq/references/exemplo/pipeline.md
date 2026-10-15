# Pipeline Pattern (PUSH/PULL)

The PUSH/PULL pattern provides one-way task distribution, ideal for work queues.

## Task Ventilator (Producer)

```rust
use rand::Rng;
use std::io::{self, BufRead};

fn main() {
    let context = zmq::Context::new();
    let sender = context.socket(zmq::PUSH).unwrap();
    sender.bind("tcp://*:5557").unwrap();

    let sink = context.socket(zmq::PUSH).unwrap();
    sink.connect("tcp://localhost:5558").unwrap();

    println!("Press Enter when the workers are ready: ");
    let stdin = io::stdin();
    stdin.lock().lines().next();

    sink.send("0", 0).unwrap();

    let mut rng = rand::thread_rng();
    let mut total_msec: u32 = 0;
    for _ in 0..100 {
        let workload: u32 = rng.gen_range(1..101);
        total_msec += workload;
        sender.send(&workload.to_string(), 0).unwrap();
    }
    println!("Total expected cost: {} msec", total_msec);
}
```

## Task Worker

```rust
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    receiver.connect("tcp://localhost:5557").unwrap();

    let sender = context.socket(zmq::PUSH).unwrap();
    sender.connect("tcp://localhost:5558").unwrap();

    loop {
        let string = receiver.recv_string(0).unwrap().unwrap();
        print!(".");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(string.parse::<u64>().unwrap()));
        sender.send("", 0).unwrap();
    }
}
```

## Task Sink (Collector)

```rust
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PULL).unwrap();
    receiver.bind("tcp://*:5558").unwrap();

    receiver.recv_bytes(0).unwrap();
    let start = Instant::now();

    for task_nbr in 0..100 {
        receiver.recv_bytes(0).unwrap();
        if task_nbr % 10 == 0 { print!(":"); } else { print!("."); }
        let _ = io::stdout().flush();
    }
    println!("\nTotal elapsed time: {:?}", start.elapsed());
}
```

## Flow Diagram

```
Ventilator -> PUSH -> [Worker 1 -> PULL] -> PUSH -> Sink
                    [Worker 2 -> PULL] -> PUSH ->
                    [Worker 3 -> PULL] -> PUSH ->
```

## Key Points

- PUSH distributes work among connected workers (load balancing)
- PULL collects results from workers
- Ventilator sends "0" to signal sink to start timing
- Workers process messages and send empty result to sink
- Sink waits for start signal, then collects all results
