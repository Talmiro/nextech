# Socket Tests

Basic socket operations: send, receive, multipart, polling.

## Message Exchange

```rust
// tests/test.rs
let (sender, receiver) = create_socketpair();
sender.send("hello", 0).unwrap();
let msg = receiver.recv_msg(0).unwrap();
assert_eq!(&msg[..], b"hello");
```

## Bytes

```rust
sender.send("data", 0).unwrap();
let bytes = receiver.recv_bytes(0).unwrap();
assert_eq!(bytes, b"data");
```

## Strings (UTF-8)

```rust
sender.send("café", 0).unwrap();
let s = receiver.recv_string(0).unwrap().unwrap();
assert_eq!(s, "café");
```

## Multipart Messages

```rust
// Send multiple parts
sender.send("part1", SNDMORE).unwrap();
sender.send("part2", 0).unwrap();

// Receive
let parts = receiver.recv_multipart(0).unwrap();
```

## Polling

```rust
let mut items = [socket.as_poll_item(zmq::POLLIN)];
zmq::poll(&mut items, 1000).unwrap();
if items[0].is_readable() {
    let msg = socket.recv_msg(0).unwrap();
}
```

## Socket Types

```rust
let types = [PAIR, PUB, SUB, REQ, REP, DEALER, ROUTER, PULL, PUSH, XPUB, XSUB, STREAM];
for t in types {
    let sock = ctx.socket(t).unwrap();
    assert_eq!(sock.get_socket_type().unwrap(), t);
}
```

## Running

```bash
cd rust-zmq && cargo test test_exchanging
cd rust-zmq && cargo test test_polling
```
