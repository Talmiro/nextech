# Query/Reply Pattern

Request/Reply pattern using Querier and Queryable for RPC-style communication.

## Queryable (Server)

```rust
use zenoh::prelude::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = zenoh::open(zenoh::config::Config::default()).await.unwrap();
    
    let queryable = session
        .declare_queryable("demo/service/time")
        .await
        .unwrap();
    
    while let Ok(query) = queryable.recv_async().await {
        println!("Received query: {}", query.selector());
        
        // Send reply
        let reply = query
            .reply()
            .put("Current time: ".to_string() + &chrono::Utc::now().to_rfc3339());
        reply.await.unwrap();
    }
    
    Ok(())
}
```

## Querier (Client)

```rust
use zenoh::prelude::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = zenoh::open(zenoh::config::Config::default()).await.unwrap();
    
    let querier = session.declare_querier("demo/service/time").await?;
    
    let mut replies = querier.get().await?;
    
    while let Ok(reply) = replies.recv_async().await {
        match reply.result() {
            Ok(sample) => {
                println!("Reply: {:?}", sample.payload());
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    
    Ok(())
}
```

## With Consolidation

```rust
use zenoh::query::ConsolidationMode;

let replies = querier
    .get()
    .consolidation(ConsolidationMode::Latest)
    .await?;
```

## With Parameters

```rust
// Client sends selector with parameters
let replies = session
    .get("demo/device/*?timeout=5000")
    .await?;

// Server receives and parses parameters
while let Ok(query) = queryable.recv_async().await {
    let params = query.parameters();
    let timeout = params.get("timeout");
}
```

## Send and Forget

```rust
let replies = querier
    .get()
    .target(zenoh::query::QueryTarget::All)
    .timeout(std::time::Duration::from_secs(5))
    .await?;
```

## See Also

- [Pub/Sub](pubsub.md) — One-way message distribution
- [Key Expressions](keyexpr.md) — Advanced key matching
