# Zenoh Examples

This file documents how to run and use Zenoh examples.

## Running Examples

```bash
# List available examples
cargo run --example --help

# Or check the examples directory
ls examples/
```

## Common Examples

### z_pub - Publisher

```rust
// examples/z_pub.rs
use zenoh::config::Config;
use zenoh::prelude::*;

#[tokio::main]
async fn main() {
    let session = zenoh::open(Config::default()).await.unwrap();
    
    let publisher = session.declare_publisher("demo//pub").await.unwrap();
    
    let mut count = 0;
    loop {
        publisher.put(format!("{}", count)).await.unwrap();
        count += 1;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
```

### z_sub - Subscriber

```rust
// examples/z_sub.rs
use zenoh::config::Config;
use zenoh::prelude::*;

#[tokio::main]
async fn main() {
    let session = zenoh::open(Config::default()).await.unwrap();
    
    let subscriber = session.declare_subscriber("demo/**").await.unwrap();
    
    while let Ok(sample) = subscriber.recv_async().await {
        println!("Received: {:?}", sample);
    }
}
```

### z_queryable - Queryable

```rust
// examples/z_queryable.rs
use zenoh::config::Config;
use zenoh::prelude::*;

#[tokio::main]
async fn main() {
    let session = zenoh::open(Config::default()).await.unwrap();
    
    let queryable = session.declare_queryable("demo/queries").await.unwrap();
    
    while let Ok(query) = queryable.recv_async().await {
        // Process query and send reply
        query.reply("ok").payload("response data").await.unwrap();
    }
}
```

### z_querier - Querier

```rust
// examples/z_querier.rs
use zenoh::config::Config;
use zenoh::prelude::*;

#[tokio::main]
async fn main() {
    let session = zenoh::open(Config::default()).await.unwrap();
    
    let replies = session.get("demo/queries").await.unwrap();
    
    while let Ok(reply) = replies.recv_async().await {
        println!("Reply: {:?}", reply);
    }
}
```

## More Examples

Other useful examples:
- `z_liveliness` - Liveliness detection
- `z_pull` - Subscriber with pull mode
- `z_info` - Session info
- `z_scout` - Scout for peers
- `z_shm` - Shared memory example

## Testing

```bash
# Run tests
cargo test

# Run specific test
cargo test my_test

# Run with logging
RUST_LOG=debug cargo test
```
