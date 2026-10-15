# Publisher-Subscriber Pattern

The PUB/SUB pattern provides one-to-many message distribution.

## Weather Server (Publisher)

```rust
use rand::Rng;

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher.bind("tcp://*:5556").unwrap();
    publisher.bind("ipc://weather.ipc").unwrap();

    let mut rng = rand::thread_rng();
    loop {
        let zipcode = rng.gen_range(0..100_000);
        let temperature = rng.gen_range(-80..135);
        let relhumidity = rng.gen_range(10..60);

        let update = format!("{:05} {} {}", zipcode, temperature, relhumidity);
        publisher.send(&update, 0).unwrap();
    }
}
```

## Weather Client (Subscriber)

```rust
use std::env;

fn main() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5556").unwrap();

    let filter = if env::args().len() > 1 { &env::args().collect::<Vec<_>>()[1] } else { "10001" };
    subscriber.set_subscribe(filter.as_bytes()).unwrap();

    let mut total_temp = 0;
    for _ in 0..100 {
        let string = subscriber.recv_string(0).unwrap().unwrap();
        let chks: Vec<i64> = string.split(' ').map(|s| s.parse().unwrap()).collect();
        let (_zipcode, temperature, _relhumidity) = (chks[0], chks[1], chks[2]);
        total_temp += temperature;
    }
    println!("Average temperature for zipcode '{}' was {}F", filter, total_temp / 100);
}
```

## Pub with Envelope

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher.bind("tcp://*:5563").unwrap();

    loop {
        publisher.send("A", zmq::SNDMORE).unwrap();
        publisher.send("We don't want to see this", 0).unwrap();
        publisher.send("B", zmq::SNDMORE).unwrap();
        publisher.send("We would like to see this", 0).unwrap();
        thread::sleep(Duration::from_millis(1));
    }
}
```

## Sub with Envelope

```rust
fn main() {
    let context = zmq::Context::new();
    let subscriber = context.socket(zmq::SUB).unwrap();
    subscriber.connect("tcp://localhost:5563").unwrap();
    subscriber.set_subscribe(b"B").unwrap();

    loop {
        let envelope = subscriber.recv_string(0).unwrap().unwrap();
        let message = subscriber.recv_string(0).unwrap().unwrap();
        println!("[{}] {}", envelope, message);
    }
}
```

## Key Points

- SUB sockets must set subscription filters using `set_subscribe()`
- Empty filter `""` subscribes to all messages
- PUB does not have delivery guarantees (slow subscriber may lose messages)
- Use envelope pattern (SNDMORE) for message filtering
- Subscriber always receives messages after subscribing (not before)
