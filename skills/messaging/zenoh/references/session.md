# Session

The `Session` is the main component of Zenoh. It holds the zenoh runtime and allows declaring publishers, subscribers, queriers, queryables, and more.

## Opening a Session

```rust
use zenoh::config::Config;
use zenoh::prelude::*;

// Default config - connects to router on localhost
let session = zenoh::open(Config::default()).await.unwrap();

// Custom config
let config = Config::from_json5(r#"{
    mode: "peer",
    listen: { endpoints: ["tcp/0.0.0.0:0"] }
}"#).unwrap();
let session = zenoh::open(config).await.unwrap();
```

## Publishing Data

### Direct Put

```rust
use zenoh::bytes::Encoding;

// Simple put
session.put("demo/key", "Hello Zenoh!").await.unwrap();

// With encoding
session.put("demo/key", "payload")
    .encoding(Encoding::TEXT_PLAIN)
    .await
    .unwrap();
```

### Using Publisher

```rust
// Declare publisher
let publisher = session.declare_publisher("demo/key").await.unwrap();

// Publish multiple times
publisher.put("message 1").await.unwrap();
publisher.put("message 2").await.unwrap();

// Delete (remove data)
publisher.delete().await.unwrap();
```

## Subscribing

```rust
// Declare subscriber
let subscriber = session.declare_subscriber("demo/**").await.unwrap();

// Receive data
while let Ok(sample) = subscriber.recv_async().await {
    println!("Received: {:?}", sample);
}
```

## Query/Reply

### Querier (client)

```rust
// Declare querier
let querier = session.declare_querier("demo/key").await.unwrap();

// Get replies
let replies = querier.get().await.unwrap();

while let Ok(reply) = replies.recv_async().await {
    if let Ok(sample) = reply.result() {
        println!("Reply: {:?}", sample);
    }
}
```

### Queryable (server)

```rust
// Declare queryable
let queryable = session.declare_queryable("demo/key").await.unwrap();

// Handle queries
while let Ok(query) = queryable.recv_async().await {
    query.reply("demo/key", "response").await.unwrap();
}
```

## Key Expression Declaration

```rust
// Declare optimized key expression
let key_expr = session.declare_keyexpr("demo/key").await.unwrap();

// Use with subscribers, publishers, etc.
let publisher = session.declare_publisher(&key_expr).await.unwrap();
```

## Session Info

```rust
// Get session ID
let zid = session.zid();
println!("Session ID: {}", zid);

// Check if closed
if !session.is_closed() {
    println!("Session is active");
}
```

## Closing Session

```rust
// Close session - undeclares all entities
session.close().await.unwrap();

// Or undeclare specific entities
session.undeclare(publisher).await.unwrap();
session.undeclare(subscriber).await.unwrap();
```

## Cloning

Session is cheap to clone - uses Arc internally:

```rust
let session1 = zenoh::open(Config::default()).await.unwrap();
let session2 = session1.clone();

// Both can be used independently
let sub = session1.declare_subscriber("demo/**").await.unwrap();
let pub = session2.declare_publisher("demo/key").await.unwrap();
```

## See Also

- [Config](config.md) - Configuration
- [Pub/Sub](pubsub.md) - Pub/Sub patterns
- [Query/Reply](reqrep.md) - Query patterns
- [Key Expressions](keyexpr.md) - Key expression matching
