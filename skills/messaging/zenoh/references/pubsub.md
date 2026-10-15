# Pub/Sub Pattern

Publisher-Subscriber pattern for one-way message distribution with hierarchical key expressions.

## Publisher (Sender)

```rust
use zenoh::prelude::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = zenoh::open(zenoh::config::Config::default()).await.unwrap();
    
    let publisher = session.declare_publisher("demo/weather/temperature").await?;
    
    publisher.put("25.5").await?;
    
    // With custom encoding
    publisher
        .put("25.5")
        .encoding(Encoding::TEXT_PLAIN)
        .await?;
    
    Ok(())
}
```

## Subscriber (Receiver)

```rust
use zenoh::prelude::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = zenoh::open(zenoh::config::Config::default()).await.unwrap();
    
    let subscriber = session.declare_subscriber("demo/weather/*").await?;
    
    while let Ok(sample) = subscriber.recv_async().await {
        println!("Received: {} = {:?}", sample.key_expr(), sample.payload());
    }
    
    Ok(())
}
```

## With QoS Options

```rust
use zenoh::qos::*;

let publisher = session
    .declare_publisher("demo/key")
    .priority(Priority::RealTime)
    .reliability(Reliability::Reliable)
    .congestion_control(CongestionControl::Block)
    .await
    .unwrap();
```

## Weather Station Example

```rust
use rand::Rng;
use zenoh::prelude::*;

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = zenoh::open(zenoh::config::Config::default()).await.unwrap();
    let mut rng = rand::rng();
    
    let publisher = session.declare_publisher("weather/station/1").await?;
    
    loop {
        let temp = rng.random_range(-80..135);
        let humidity = rng.random_range(10..60);
        
        publisher
            .put(format!("{{\"temp\": {}, \"humidity\": {}}}", temp, humidity))
            .await?;
        
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
```

## Matching Status

Check if subscribers are available:

```rust
let publisher = session.declare_publisher("demo/key").await?;

if publisher.matching_status().await?.matching() {
    println!("Subscribers are listening!");
}
```

## See Also

- [Query/Reply](reqrep.md) — Two-way communication
- [Key Expressions](keyexpr.md) — Advanced key matching
- [Scouting](scouting.md) — Auto-discovery
