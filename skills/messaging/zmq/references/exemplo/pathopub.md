# Pathological Publisher

Publisher that sends many topics then random updates - tests subscriber handling.

```rust
// examples/zguide/pathopub/main.rs
use std::env;
use std::thread::sleep;
use std::time::Duration;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    let address = if env::args().len() == 2 { &env::args().collect::<Vec<_>>()[1] } else { "tcp://*:5556" };
    publisher.bind(address).unwrap();

    sleep(Duration::from_millis(1000));

    for topic_nbr in 0..1000 {
        publisher.send(&format!("{:03}", topic_nbr), zmq::SNDMORE).unwrap();
        publisher.send("Save Roger", 0).unwrap();
    }

    let mut rng = rand::thread_rng();
    let topic_range = Uniform::new(0, 1000);
    loop {
        sleep(Duration::from_millis(1000));
        publisher.send(&format!("{:03}", topic_range.sample(&mut rng)), zmq::SNDMORE).unwrap();
        publisher.send("Off with his head!", 0).unwrap();
    }
}
```

## Key Points

- Sends 1000 initial topics on startup
- Then sends one random topic per second
- Tests slow subscriber handling
- Each message is topic + data (multipart)
- Useful for testing pub/sub performance
