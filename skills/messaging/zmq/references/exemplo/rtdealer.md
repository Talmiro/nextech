# Router to Dealer

Router to DEALER pattern for load balancing.

```rust
// examples/zguide/rtdealer/main.rs
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};

fn worker_task() {
    let context = zmq::Context::new();
    let worker = context.socket(zmq::DEALER).unwrap();
    let identity: Vec<_> = (0..10).map(|_| rand::random::<u8>()).collect();
    worker.set_identity(&identity).unwrap();
    worker.connect("tcp://localhost:5671").unwrap();

    let mut total = 0;
    loop {
        worker.send("", zmq::SNDMORE).unwrap();
        worker.send("Hi boss!", 0).unwrap();

        worker.recv_bytes(0).unwrap();
        let workload = worker.recv_string(0).unwrap().unwrap();
        if workload == "Fired!" { break; }
        total += 1;
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1..500)));
    }
}

fn main() {
    let context = zmq::Context::new();
    let broker = context.socket(zmq::ROUTER).unwrap();
    broker.bind("tcp://*:5671").unwrap();

    for _ in 0..10 {
        thread::spawn(|| worker_task());
    }

    let start_time = Instant::now();
    let mut workers_fired = 0;
    loop {
        let identity = broker.recv_bytes(0).unwrap();
        broker.send(&identity, zmq::SNDMORE).unwrap();
        broker.recv_bytes(0).unwrap();
        broker.recv_bytes(0).unwrap();
        broker.send("", zmq::SNDMORE).unwrap();

        if start_time.elapsed() < Duration::new(5, 0) {
            broker.send("Work harder", 0).unwrap();
        } else {
            broker.send("Fired!", 0).unwrap();
            workers_fired += 1;
            if workers_fired >= 10 { break; }
        }
    }
}
```

## Key Points

- Uses DEALER instead of REQ for workers
- DEALER can send/receive independently
- Workers don't need identities but ROUTER adds them
- Router-to-Dealer provides async request handling
- Can have multiple in-flight requests per worker
