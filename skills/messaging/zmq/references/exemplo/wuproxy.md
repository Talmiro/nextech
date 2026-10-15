# Weather Update Proxy

XSUB/XPUB proxy for forwarding weather updates.

```rust
// examples/zguide/wuproxy/main.rs
fn main() {
    let context = zmq::Context::new();
    let frontend = context.socket(zmq::XSUB).unwrap();
    let backend = context.socket(zmq::XPUB).unwrap();

    frontend.connect("tcp://192.168.55.210:5556").unwrap();
    backend.bind("tcp://10.1.1.0:8100").unwrap();

    zmq::proxy(&frontend, &backend).expect("failed proxying");
}
```

## Key Points

- XSUB to connect to upstream publisher
- XPUB to bind for downstream subscribers
- Simple proxy using zmq::proxy
- Subscriptions flow through from XPUB to XSUB
- Used to bridge weather updates across networks
