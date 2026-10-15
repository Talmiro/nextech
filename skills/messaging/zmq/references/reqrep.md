# REQ/REP Pattern

Request-Reply - synchronous request-response.

## Requester (REQ)

```rust
let req = ctx.socket(zmq::REQ).unwrap();
req.connect("tcp://localhost:5555").unwrap();
req.send("request", 0).unwrap();
let reply = req.recv_string(0).unwrap().unwrap();
```

## Replier (REP)

```rust
let rep = ctx.socket(zmq::REP).unwrap();
rep.bind("tcp://*:5555").unwrap();
let request = rep.recv_string(0).unwrap().unwrap();
rep.send("response", 0).unwrap();
```

## Key Points

- REQ must send first, then receive
- REP must receive first, then send
- Synchronous - blocks until peer responds
- Cannot fork REQ sockets
- Use for simple RPC
