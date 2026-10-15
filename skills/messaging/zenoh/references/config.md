# Config

The `Config` struct is used to configure the Zenoh session. It provides various options for network, scouting, transport, security, and more.

## Basic Usage

```rust
use zenoh::config::Config;

// Default configuration (client mode, connects to router on localhost)
let config = Config::default();

// From JSON5 string
let config = Config::from_json5(r#"{
    mode: "peer",
    connect: {
        endpoints: ["tcp/127.0.0.1:7447"]
    }
}"#).await.unwrap();

// From file
let config = Config::from_file("path/to/config.json5").await.unwrap();

// From environment
let config = Config::from_env().await.unwrap();
```

## Common Configurations

### Client Mode (connect to router)

```rust
let config = Config::from_json5(r#{
    mode: "client",
    connect: {
        endpoints: ["tcp/192.168.1.1:7447"]
    }
}"#).unwrap();
```

### Peer Mode (mesh network)

```rust
let config = Config::from_json5(r#{
    mode: "peer",
    listen: {
        endpoints: ["tcp/0.0.0.0:7447"]
    },
    scouting: {
        multicast: {
            enabled: true,
            autoconnect: { peer: ["router", "peer"] }
        }
    }
}"#).unwrap();
```

### Router Mode

```rust
let config = Config::from_json5(r#{
    mode: "router",
    listen: {
        endpoints: ["tcp/0.0.0.0:7447"]
    }
}"#).unwrap();
```

## Modifying Config

```rust
let config = Config::default();

// Insert JSON5 values
config.insert_json5("connect/endpoints", r#"["tcp/localhost:7447"]"#).unwrap();

// Get JSON values
let endpoints = config.get_json("connect/endpoints").unwrap();
println!("Endpoints: {:?}", endpoints);

// Remove values
config.remove("connect/endpoints").unwrap();
```

## Environment Variable

The config can also be loaded from `ZENOH_CONFIG_FILE` environment variable:

```rust
// Uses ZENOH_CONFIG_PATH_ENV or ZENOH_CONFIG_FILE
let config = Config::from_env().await.unwrap();
```

## See Also

- [Session](session.md) - The main Zenoh session
- [Pub/Sub](pubsub.md) - Publishing and subscribing
- [Query/Reply](reqrep.md) - Request/reply patterns
