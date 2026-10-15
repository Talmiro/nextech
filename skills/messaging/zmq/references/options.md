# Socket Options Tests

Common socket configuration options.

## High Water Marks

```rust
sock.set_sndhwm(500).unwrap();
sock.set_rcvhwm(500).unwrap();
```

## Timeouts (milliseconds)

```rust
sock.set_rcvtimeo(5000).unwrap();
sock.set_sndtimeo(5000).unwrap();
```

## Identity & Routing

```rust
sock.set_identity(b"worker1").unwrap();
assert_eq!(sock.get_identity().unwrap(), b"worker1");
```

## Linger

```rust
sock.set_linger(1000).unwrap();  // ms to wait on close
```

## Reconnection

```rust
sock.set_reconnect_ivl(100).unwrap();
sock.set_reconnect_ivl_max(5000).unwrap();
```

## TCP Options

```rust
sock.set_tcp_keepalive(1).unwrap();
sock.set_tcp_keepalive_cnt(5).unwrap();
sock.set_tcp_keepalive_idle(60).unwrap();
sock.set_tcp_keepalive_intvl(10).unwrap();
sock.set_ipv6(true).unwrap();
```

## Buffers

```rust
sock.set_sndbuf(8192).unwrap();
sock.set_rcvbuf(8192).unwrap();
```

## Other Options

```rust
sock.set_rate(100).unwrap();
sock.set_backlog(50).unwrap();
sock.set_multicast_hops(3).unwrap();
sock.set_maxmsgsize(1024).unwrap();
sock.set_conflate(true).unwrap();
```

## Running

```bash
cd rust-zmq && cargo test test_getset_
```
