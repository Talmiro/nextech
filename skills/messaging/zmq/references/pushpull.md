# PUSH/PULL Pattern

Pipeline - one-way task distribution.

## PUSH (Producer)

```rust
let push = ctx.socket(zmq::PUSH).unwrap();
push.bind("tcp://*:5555").unwrap();
for i in 0..10 {
    push.send(&format!("task{}", i), 0).unwrap();
}
```

## PULL (Consumer)

```rust
let pull = ctx.socket(zmq::PULL).unwrap();
pull.connect("tcp://localhost:5555").unwrap();
while let Ok(msg) = pull.recv_string(0) {
    println!("Received: {}", msg);
}
```

## Key Points

- PUSH distributes work (load balancing)
- PULL collects work
- One-way: no reply
- Workers process in parallel
- Used for task queues
