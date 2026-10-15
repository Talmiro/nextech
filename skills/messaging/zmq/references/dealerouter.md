# DEALER/ROUTER Pattern

Asynchronous request-response with routing.

## ROUTER

```rust
let router = ctx.socket(zmq::ROUTER).unwrap();
router.bind("tcp://*:5555").unwrap();

// First frame is always client identity
let id = router.recv_bytes(0).unwrap();
let msg = router.recv_string(0).unwrap().unwrap();

// Reply to specific client
router.send(&id, zmq::SNDMORE).unwrap();
router.send("response", 0).unwrap();
```

## DEALER

```rust
let dealer = ctx.socket(zmq::DEALER).unwrap();
dealer.connect("tcp://localhost:5555").unwrap();
dealer.send("request", 0).unwrap();
let reply = dealer.recv_string(0).unwrap().unwrap();
```

## Key Points

- ROUTER tracks client identity
- DEALER is async - can send/receive independently
- Can handle multiple concurrent clients
- Use set_identity() to identify clients
- Often combined with proxy()
