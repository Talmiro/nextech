# Last Value Cache

XSUB/XPUB based cache that sends last value to new subscribers.

```rust
// examples/zguide/lvcache/main.rs
use std::collections::HashMap;
use std::str::from_utf8;

fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::SUB).unwrap();
    let backend = context.socket(zmq::XPUB).unwrap();

    frontend.connect("tcp://localhost:5557").unwrap();
    backend.bind("tcp://*:5558").unwrap();
    frontend.set_subscribe(b"").unwrap();

    let mut cache = HashMap::new();

    loop {
        let mut items = [
            frontend.as_poll_item(zmq::POLLIN),
            backend.as_poll_item(zmq::POLLIN),
        ];
        if zmq::poll(&mut items, 1000).is_err() { break; }

        if items[0].is_readable() {
            let topic = frontend.recv_msg(0).unwrap();
            let current = frontend.recv_msg(0).unwrap();
            cache.insert(topic.to_vec(), current.to_vec());
            backend.send(topic, zmq::SNDMORE).unwrap();
            backend.send(current, 0).unwrap();
        }

        if items[1].is_readable() {
            let event = backend.recv_msg(0).unwrap();
            if event[0] == 1 {
                let topic = &event[1..];
                println!("Sending cached topic {}", from_utf8(topic).unwrap());
                if let Some(previous) = cache.get(topic) {
                    backend.send(topic, zmq::SNDMORE).unwrap();
                    backend.send(previous, 0).unwrap();
                }
            }
        }
    }
}
```

## Key Points

- XPUB sends subscription events to the application
- Event[0] = 1 means subscribe, 0 means unsubscribe
- Cache stores last value for each topic
- New subscribers receive cached value immediately
- Uses XSUB to connect to upstream publisher
