# WhatAmI, WhatAmIMatcher, ZenohId

These are core configuration and identity types in Zenoh.

## WhatAmI

The `WhatAmI` enum defines the node type in a Zenoh network.

### Variants

```rust
use zenoh::config::WhatAmI;

// Router - maintains network topology, doesn't discover nodes automatically
WhatAmI::Router  // = 1

// Peer - mesh networking, discovers and connects to other peers
WhatAmI::Peer    // = 2 (default)

// Client - connects to a single router/gateway
WhatAmI::Client  // = 4
```

### Using in Config

```rust
use zenoh::config::Config;

// Router mode
let config = Config::from_json5(r#"{
    mode: "router",
    listen: { endpoints: ["tcp/0.0.0.0:7447"] }
}"#).unwrap();

// Peer mode (default)
let config = Config::from_json5(r#"{
    mode: "peer",
    listen: { endpoints: ["tcp/0.0.0.0:0"] },
    scouting: { multicast: { enabled: true } }
}"#).unwrap();

// Client mode
let config = Config::from_json5(r#"{
    mode: "client",
    connect: { endpoints: ["tcp/192.168.1.1:7447"] }
}"#).unwrap();
```

### Converting

```rust
use zenoh::config::WhatAmI;

// From string
let mode: WhatAmI = "peer".parse().unwrap();

// To string
let s = WhatAmI::Peer.to_str();  // "peer"

// Using Display
let s = format!("{}", WhatAmI::Router);
```

## WhatAmIMatcher

Combine multiple `WhatAmI` values for matching:

```rust
use zenoh::config::{WhatAmI, WhatAmIMatcher};

// Match multiple types
let matcher: WhatAmIMatcher = WhatAmI::Router | WhatAmI::Peer;

// From WhatAmI
let matcher: WhatAmIMatcher = WhatAmI::Client.into();

// Match any
let any: WhatAmIMatcher = WhatAmI::Router | WhatAmI::Peer | WhatAmI::Client;
```

## ZenohId

The unique identifier for a Zenoh node:

```rust
use zenoh::session::ZenohId;

// Get from session
let session = zenoh::open(Config::default()).await.unwrap();
let zid: ZenohId = session.zid();

println!("Session ID: {}", zid);

// Default - random 128-bit ID
let default_id = ZenohId::default();

// From integer
let id = ZenohId::from(0x1234567890abcdef_u128);

// From string
let id: ZenohId = "1234567890abcdef".parse().unwrap();
```

### Properties

```rust
let zid = session.zid();

// As/u128
let n: u128 = zid.into();

// Display
println!("ID: {}", zid);

// UUID format
println!("UUID: {}", zid.to_uuid_string());
```

## Setting Custom Zenoh ID

```rust
// In config
let config = Config::from_json5(r#"{
    id: "1234567890abcdef"
}"#).unwrap();

// Or programmatically
// Note: ID must be set before opening session
```

## Node Mode Summary

| Mode | Discovery | Connections | Use Case |
|------|-----------|-------------|----------|
| Router | Static config | Accepts many | Gateway, broker |
| Peer | Auto-discovery | Mesh | Distributed systems |
| Client | None | Single gateway | Edge devices |

## See Also

- [Config](config.md) - Configuration
- [Session](session.md) - Session with zid
- [Scouting](scouting.md) - Peer discovery
