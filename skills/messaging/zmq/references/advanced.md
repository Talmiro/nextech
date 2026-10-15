# Advanced Tests

Monitor, poll integration, external event loops.

## Socket Monitor

```rust
// tests/monitor.rs
client.monitor("inproc://client mon", zmq::SocketEvent::ALL as i32).unwrap();
let mut mon = ctx.socket(zmq::PAIR).unwrap();
mon.connect("inproc://client mon").unwrap();

// Events: CONNECTED, LISTENING, ACCEPTED, DISCONNECTED, CLOSED
let event = get_monitor_event(&mut mon).unwrap();
assert_eq!(event, zmq::SocketEvent::CONNECTED);
```

## Poll with Unix FD

```rust
// tests/poll/unix.rs
use nix::unistd;
let (read, write) = unistd::pipe().unwrap();
let poll_item = zmq::PollItem::from_fd(read, zmq::POLLIN);
let mut items = [poll_item];
zmq::poll(&mut items, 1000).unwrap();
assert!(items[0].is_readable());
```

## External Event Loop

```rust
// tests/unix/connection.rs
use nix::poll::{poll, PollFlags};

let fd = socket.get_fd().unwrap();
let mut poll_fd = poll::PollFd::new(fd, PollFlags::POLLIN);
poll::poll(&mut [poll_fd], -1).unwrap();

// Check socket events
let events = socket.get_events().unwrap() as zmq::PollEvents;
```

## Running

```bash
cd rust-zmq && cargo test monitor
cd rust-zmq && cargo test test_pipe_poll
cd rust-zmq && cargo test test_external_poll
```
