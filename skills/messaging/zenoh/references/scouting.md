# Scouting (Discovery)

Zenoh supports auto-discovery of peers and routers on the network.

## Scout for Peers

```rust
use zenoh::prelude::*;
use zenoh::scouting::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scout = zenoh::scout(WhatAmI::Peer)
        .await
        .unwrap();
    
    while let Ok(hello) = scout.recv_async().await {
        println!("Found peer: {:?}", hello);
    }
}
```

## Scout for Routers

```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut scout = zenoh::scout(WhatAmI::Router)
        .await
        .unwrap();
    
    while let Ok(hello) = scout.recv_async().await {
        println!("Found router: {:?}", hello);
    }
}
```

## Scout for Both

```rust
let mut scout = zenoh::scout(WhatAmI::Peer | WhatAmI::Router).await?;
```

## Scout with Timeout

```rust
use std::time::Duration;

let mut scout = zenoh::scout(WhatAmI::Peer)
    .await
    .unwrap();

// Get first peer found
if let Ok(hello) = tokio::time::timeout(Duration::from_secs(5), scout.recv_async()).await {
    println!("Found: {:?}", hello);
}
```

## Connect to Discovered Peer

```rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First, scout for peers
    let mut scout = zenoh::scout(WhatAmI::Peer).await.unwrap();
    
    if let Ok(hello) = scout.recv_async().await {
        // Connect to the discovered peer
        let mut config = zenoh::config::Config::default();
        config.insert_json5("connect/endpoints", &format!(r#"["{}"]"#, hello.locator()))
            .unwrap();
        
        let session = zenoh::open(config).await.unwrap();
        // Use session...
    }
    
    Ok(())
}
```

## Configuration for Auto-Scout

```rust
let mut config = zenoh::config::Config::default();

// Enable scouting
config.set_scouting(true).unwrap();
config.set_scouting_delay(Duration::from_secs(1)).unwrap();

// Connect to discovered peers automatically
config.insert_json5("connect/endpoints", r#"["auto/discovery"]"#).unwrap();
```

## Hello Message

```rust
struct Hello {
    whatami: WhatAmI,      // What the peer is (Peer/Router/Client)
    zid: ZenohId,          // Unique Zenoh ID
    locator: Locator,      // How to connect
    metadata: Properties,  // Optional metadata
}
```

## See Also

- [Pub/Sub](pubsub.md) — Using discovered peers
- [Query/Reply](reqrep.md) — Using discovered queryables
