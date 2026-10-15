# Pathological Subscriber

Subscriber that subscribes to one random topic.

```rust
// examples/zguide/pathosub/main.rs
use std::env;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    let address = if env::args().len() == 2 { &env::args().collect::<Vec<_>>()[1] } else { "tcp://localhost:5556" };
    subscriber.connect(address).unwrap();

    let mut rng = rand::thread_rng();
    let topic_range = Uniform::new(0, 1000);
    let subscription = format!("{:03}", topic_range.sample(&mut rng)).into_bytes();
    subscriber.set_subscribe(&subscription).unwrap();

    loop {
        let topic = subscriber.recv_msg(0).unwrap();
        let data = subscriber.recv_msg(0).unwrap();
        assert_eq!(&topic[..], &subscription[..]);
        println!("{}", std::str::from_utf8(&data).unwrap());
    }
}
```

## Key Points

- Subscribes to a random topic from 000-999
- Receives only messages matching the subscription
- Used with pathopub to test pub/sub handling
- Each message is multipart (topic + data)
