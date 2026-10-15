# Request-Reply Pattern

The REQ/REP pattern provides synchronous request-response communication.

## Hello World Server

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    responder.bind("tcp://*:5555").unwrap();

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Received {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}
```

## Hello World Client

```rust
fn main() {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    requester.connect("tcp://localhost:5555").unwrap();

    let mut msg = zmq::Message::new();
    for request_nbr in 0..10 {
        println!("Sending Hello {}...", request_nbr);
        requester.send("Hello", 0).unwrap();
        requester.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}
```

## RRClient

```rust
fn main() {
    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    requester.connect("tcp://localhost:5559").unwrap();

    for request_nbr in 0..10 {
        requester.send("Hello", 0).unwrap();
        let message = requester.recv_msg(0).unwrap();
        println!("Received reply {} {}", request_nbr, message.as_str().unwrap());
    }
}
```

## RRWorker

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    responder.connect("tcp://localhost:5560").unwrap();

    loop {
        let string = responder.recv_string(0).unwrap().unwrap();
        println!("Received request:{}", string);
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}
```

## RRBroker

```rust
fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::ROUTER).unwrap();
    let backend = context.socket(zmq::DEALER).unwrap();

    frontend.bind("tcp://*:5559").unwrap();
    backend.bind("tcp://*:5560").unwrap();

    loop {
        let mut items = [
            frontend.as_poll_item(zmq::POLLIN),
            backend.as_poll_item(zmq::POLLIN),
        ];
        zmq::poll(&mut items, -1).unwrap();

        if items[0].is_readable() {
            loop {
                let message = frontend.recv_msg(0).unwrap();
                let more = message.get_more();
                backend.send(message, if more { zmq::SNDMORE } else { 0 }).unwrap();
                if !more { break; }
            }
        }
        if items[1].is_readable() {
            loop {
                let message = backend.recv_msg(0).unwrap();
                let more = message.get_more();
                frontend.send(message, if more { zmq::SNDMORE } else { 0 }).unwrap();
                if !more { break; }
            }
        }
    }
}
```

## Key Points

- REQ must initiate communication (send first)
- REP blocks until it receives a request
- Use ROUTER/DEALER for async request-response
- Use a broker (ROUTER/DEALER) to connect multiple clients to workers
