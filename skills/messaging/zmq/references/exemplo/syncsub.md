# Synchronized Subscriber

Subscriber that syncs with publisher before receiving updates.

```rust
// examples/zguide/syncsub/main.rs
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();

    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5561").unwrap();
    subscriber.set_subscribe(b"").unwrap();
    thread::sleep(Duration::from_millis(1));

    let syncclient = context.socket(zmq::REQ).unwrap();
    syncclient.connect("tcp://localhost:5562").unwrap();
    syncclient.send("", 0).unwrap();
    syncclient.recv_msg(0).unwrap();

    let mut update_nbr = 0;
    loop {
        let message = subscriber.recv_string(0).unwrap().unwrap();
        if message == "END" { break; }
        update_nbr += 1;
    }
    println!("Received {} updates", update_nbr);
}
```

## Key Points

- First connects SUB socket and sleeps briefly
- Then uses REQ socket to sync with publisher
- Waits for sync response before receiving
- Counts updates until END message
- Used with syncpub for synchronized pub/sub
