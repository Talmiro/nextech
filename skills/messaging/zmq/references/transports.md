# Transport Tests

Connections over inproc, TCP, IPC.

## In-Process

```rust
// tests/connection.rs
let push = ctx.socket(zmq::PUSH).unwrap();
push.bind("inproc://myendpoint").unwrap();

let pull = ctx.socket(zmq::PULL).unwrap();
pull.connect("inproc://myendpoint").unwrap();

push.send("message", 0).unwrap();
let msg = pull.recv_msg(0).unwrap();
```

## TCP

```rust
let server = ctx.socket(zmq::REP).unwrap();
server.bind("tcp://127.0.0.1:*").unwrap();
let endpoint = server.get_last_endpoint().unwrap().unwrap();

let client = ctx.socket(zmq::REQ).unwrap();
client.connect(&endpoint).unwrap();
```

## IPC (Unix)

```rust
server.bind("ipc:///tmp/zmqtest").unwrap();
client.connect("ipc:///tmp/zmqtest").unwrap();
```

## Disconnect

```rust
let endpoint = receiver.get_last_endpoint().unwrap().unwrap();
sender.disconnect(&endpoint).unwrap();
```

## Running

```bash
cd rust-zmq && cargo test test_inproc
cd rust-zmq && cargo test test_tcp
```
