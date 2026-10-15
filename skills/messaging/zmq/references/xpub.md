# XPUB/XSUB Pattern

Extended PUB/SUB - allows detecting subscription events.

## XPUB (Extended Publisher)

```rust
let xpub = ctx.socket(zmq::XPUB).unwrap();
xpub.bind("tcp://*:5556").unwrap();

// Receive subscription events
let event = xpub.recv_msg(0).unwrap();
if event[0] == 1 {  // 1=subscribe, 0=unsubscribe
    let topic = &event[1..];
    println!("Subscribed to: {:?}", topic);
}
```

## XSUB (Extended Subscriber)

```rust
let xsub = ctx.socket(zmq::XSUB).unwrap();
xsub.connect("tcp://localhost:5556").unwrap();
xsub.set_subscribe(b"topic").unwrap();
```

## Key Points

- XPUB receives subscription events
- XSUB can send subscription messages programmatically
- Used for building proxies and caches
