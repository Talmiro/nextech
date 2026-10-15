# Basic Tests

Core functionality: context, messages, errors, capabilities.

## Context IO Threads

```rust
// tests/context.rs
let ctx = zmq::Context::new();
assert_eq!(ctx.get_io_threads().unwrap(), zmq_sys::ZMQ_IO_THREADS_DFLT as i32);
ctx.set_io_threads(4).unwrap();
assert_eq!(ctx.get_io_threads().unwrap(), 4);
```

## Message Comparison

```rust
// tests/message.rs
let msg1 = Message::from("hello");
let msg2 = Message::from("hello");
assert_eq!(msg1, msg2);
```

## Z85 Encoding

```rust
// tests/z85.rs
let encoded = z85_encode(&data).unwrap();
let decoded = z85_decode(&encoded).unwrap();
assert_eq!(data, decoded);
```

## Error Handling

```rust
// tests/error.rs
let error = Error::from_raw(errno::EINTR);
assert_eq!(error, Error::EINTR);
```

## Capability Detection

```rust
// tests/has.rs
assert!(zmq::has("ipc").is_some());
assert!(zmq::has("curve").is_some());
```

## Running

```bash
cd rust-zmq && cargo test
```
