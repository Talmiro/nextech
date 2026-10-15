# PUB/SUB Pattern

Publisher-Subscriber - one-to-many message distribution.

## Publisher

```rust
let pub = ctx.socket(zmq::PUB).unwrap();
pub.bind("tcp://*:5556").unwrap();
pub.send("topic", zmq::SNDMORE).unwrap();
pub.send("message", 0).unwrap();
```

## Subscriber

```rust
let sub = ctx.socket(zmq::SUB).unwrap();
sub.connect("tcp://localhost:5556").unwrap();
sub.set_subscribe(b"topic").unwrap();  // filter
let msg = sub.recv_string(0).unwrap().unwrap();
```

## Key Points

- PUB: sender, no receive
- SUB: receiver, no send
- Filter with set_subscribe()
- Empty filter = receive all
- Slow subscribers may lose messages
