# EndPoint, Locator, Notifier

These are configuration-related types in Zenoh.

## EndPoint

An endpoint represents a full connection string with protocol, address, metadata, and config.

### Format

```
<protocol>/<address>[?<metadata>][#<config>]
```

### Creating EndPoints

```rust
use zenoh::config::EndPoint;

// From string (parse)
let ep: EndPoint = "tcp/127.0.0.1:7447".parse().unwrap();
let ep: EndPoint = "tcp/192.168.1.1:7447#iface=eth0".parse().unwrap();

// With priority metadata
let ep: EndPoint = "tcp/localhost:7447?prio=1-3".parse().unwrap();

// With reliability
let ep: EndPoint = "tcp/localhost:7447?prio=6-7;rel=0".parse().unwrap();

// From parts
let ep = EndPoint::new("tcp", "localhost:7447", "", "").unwrap();
```

### Working with EndPoints

```rust
let ep: EndPoint = "tcp/192.168.1.1:7447#iface=eth0".parse().unwrap();

// Get parts
let protocol = ep.protocol();
let address = ep.address();
let metadata = ep.metadata();
let config = ep.config();

// Convert to locator
let locator: zenoh::config::Locator = ep.to_locator();
let locator_str: String = locator.into();
```

### Common Endpoints

```rust
// TCP
"tcp/127.0.0.1:7447"
"tcp/localhost:7447"

// UDP
"udp/127.0.0.1:7447"

// TLS
"tls/127.0.0.1:7447"

// QUIC
"quic/127.0.0.1:7447"

// WebSocket
"ws/127.0.0.1:7447"

// Unix socket
"unixsock-stream/path/to/socket"
```

## Locator

A locator is similar to EndPoint but without the config portion.

### Format

```
<protocol>/<address>[?<metadata>]
```

### Creating Locators

```rust
use zenoh::config::Locator;

// From string
let loc: Locator = "tcp/127.0.0.1:7447".parse().unwrap();

// From parts
let loc = Locator::new("tcp", "localhost:7447", "").unwrap();

// Convert from EndPoint
let ep: EndPoint = "tcp/127.0.0.1:7447".parse().unwrap();
let loc: Locator = ep.to_locator();
```

### Working with Locators

```rust
let loc: Locator = "tcp/192.168.1.1:7447?dscp=0x08".parse().unwrap();

let protocol = loc.protocol();
let address = loc.address();
let metadata = loc.metadata();

// Convert to endpoint
let ep = loc.to_endpoint();
```

## Notifier

A wrapper for Config that allows subscribing to configuration changes. This is an **unstable** feature.

### Enabling

```toml
[dependencies]
zenoh = { version = "1.0", features = ["unstable"] }
```

### Getting Notifier from Session

```rust
use zenoh::prelude::*;

// Get config notifier from session
let session = zenoh::open(Config::default()).await.unwrap();
let config = session.config();
```

### Reading Config

```rust
let config = session.config();

// Get value as JSON string
let endpoints = config.get("connect/endpoints").unwrap();
println!("Endpoints: {:?}", endpoints);

// Get specific nested value
let timeout = config.get("scouting/timeout").unwrap();
```

### Modifying Config

```rust
let config = session.config();

// Insert JSON5 value
config.insert_json5("connect/endpoints", r#"["tcp/localhost:7447"]"#).unwrap();

// Remove value
config.remove("connect/endpoints").unwrap();
```

### Using IConfig Trait

```rust
use zenoh_config::IConfig;

let config = session.config();

// Get as string
let value: String = config.get("mode").unwrap();

// Insert
config.insert_json5("mode", "\"peer\"").unwrap();

// Get as JSON
let json = config.to_json();
```

## Using in Config

```rust
use zenoh::config::Config;

// Connect to specific endpoints
let config = Config::from_json5(r#"{
    mode: "client",
    connect: {
        endpoints: [
            "tcp/192.168.1.1:7447",
            "tcp/192.168.1.2:7447"
        ]
    }
}"#).unwrap();

// Listen on endpoints
let config = Config::from_json5(r#"{
    mode: "peer",
    listen: {
        endpoints: ["tcp/0.0.0.0:7447"]
    }
}"#).unwrap();

// With interface selection
let config = Config::from_json5(r#"{
    connect: {
        endpoints: ["tcp/192.168.0.1:7447#iface=eth0"]
    }
}"#).unwrap();

// With TCP buffer sizes
let config = Config::from_json5(r#"{
    connect: {
        endpoints: ["tcp/192.168.0.1:7447#so_sndbuf=65000;so_rcvbuf=65000"]
    }
}"#).unwrap();
```

## See Also

- [Config](config.md) - Main configuration struct
- [Session](session.md) - Session with config access
- [Scouting](scouting.md) - Peer discovery
