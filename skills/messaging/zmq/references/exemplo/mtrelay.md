# Multithreaded Relay

PAIR sockets for inproc signaling between threads.

```rust
// examples/zguide/mtrelay/main.rs
use std::thread;

fn step1(context: &zmq::Context) {
    let xmitter = context.socket(zmq::PAIR).unwrap();
    xmitter.connect("inproc://step2").unwrap();
    println!("Step 1 ready, signalling step 2");
    xmitter.send("READY", 0).unwrap();
}

fn step2(context: &zmq::Context) {
    let receiver = context.socket(zmq::PAIR).unwrap();
    receiver.bind("inproc://step2").unwrap();
    let ctx = context.clone();
    thread::spawn(move || step1(&ctx));

    receiver.recv_msg(0).unwrap();

    let xmitter = context.socket(zmq::PAIR).unwrap();
    xmitter.connect("inproc://step3").unwrap();
    println!("Step 2 ready, signalling step 3");
    xmitter.send("READY", 0).unwrap();
}

fn main() {
    let context = zmq::Context::new();
    let receiver = context.socket(zmq::PAIR).unwrap();
    receiver.bind("inproc://step3").unwrap();
    thread::spawn(move || step2(&context));
    receiver.recv_msg(0).unwrap();
    println!("Test successful!");
}
```

## Key Points

- Uses PAIR sockets with inproc:// transport
- PAIR is bidirectional but only connects to one peer
- Must bind before connect in the thread sequence
- Inproc is intra-process, very low latency
- Useful for signaling between threads
