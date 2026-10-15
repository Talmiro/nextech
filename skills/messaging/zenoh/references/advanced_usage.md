# Advanced Usage

This file documents advanced Zenoh features and patterns.

## Batching

Zenoh supports batch publishing for higher throughput:

```rust
use zenoh::prelude::*;

// Create batch of samples
let mut batch = Vec::new();

for i in 0..100 {
    let sample = Sample::new("demo/key", format!("data {}", i));
    batch.push(sample);
}

// Publish batch
session.put_batch(batch).await.unwrap();
```

## Late Join

Subscribe to historical data with late join:

```rust
let subscriber = session.declare_subscriber("demo/**")
    .periodicity(Duration::from_secs(10))  // Enable late join with period
    .await
    .unwrap();
```

## Congestion Control

Control how publishers handle backpressure:

```rust
// Block when queue is full
let publisher = session.declare_publisher("demo/key")
    .congestion_control(CongestionControl::Block)
    .await
    .unwrap();

// Drop messages when queue is full
let publisher = session.declare_publisher("demo/key")
    .congestion_control(CongestionControl::Drop)
    .await
    .unwrap();
```

## Priority Levels

Control message priority (1=highest, 7=lowest):

```rust
let publisher = session.declare_publisher("demo/key")
    .priority(Priority::RealTime)  // 1
    .await
    .unwrap();
```

## Reliability

Configure reliability for subscribers:

```rust
// Best effort (may miss messages)
let subscriber = session.declare_subscriber("demo/**")
    .reliability(BestEffort)
    .await
    .unwrap();

// Reliable (guaranteed delivery)
let subscriber = session.declare_subscriber("demo/**")
    .reliability(Reliable)
    .await
    .unwrap();
```

## Query Consolidation

Configure how query replies are consolidated:

```rust
use zenoh::query::QueryConsolidation;

// Latest value wins (good for cached data)
let consolidation = QueryConsolidation::latest();

// Full history (good for time-series)
let consolidation = QueryConsolidation::full();

// No consolidation (all replies)
let consolidation = QueryConsolidation::none();

let replies = session.get("demo/key")
    .consolidation(consolidation)
    .await
    .unwrap();
```

## Time-Based Queries

Query historical data with time bounds:

```rust
use zenoh::query::{TimeRange, TimeExpr};

let time_range = TimeRange::new(
    "2024-01-01T00:00:00Z",
    "2024-01-02T00:00:00Z"
)?;

let replies = session.get("demo/key")
    .time_range(time_range)
    .await
    .unwrap();
```

## Matching Status

Check if subscribers are matching a key:

```rust
let publisher = session.declare_publisher("demo/key").await.unwrap();

// Check if any subscribers match
let matching = publisher.matching_status().await?;

if matching.matched() {
    println!("Subscribers are listening!");
}
```

## Liveliness

Monitor node availability:

```rust
// Declare liveliness token
let token = session.liveliness_declare("node/leader").await.unwrap();

// Subscribe to liveliness changes
let subscriber = session.liveliness_subscribe("node/**").await.unwrap();

// Check token status
token.refresh().await.unwrap();
```

## Transport Events

Monitor transport connections:

```rust
let listener = session.transport_events()
    .await
    .unwrap();

while let Ok(event) = listener.recv_async().await {
    match event {
        TransportEvent::Established(transport) => {
            println!("Transport established: {:?}", transport);
        }
        TransportEvent::Closed(transport) => {
            println!("Transport closed: {:?}", transport);
        }
    }
}
```

## Link Events

Monitor individual link events:

```rust
let listener = session.link_events()
    .await
    .unwrap();

while let Ok(event) = listener.recv_async().await {
    println!("Link event: {:?}", event);
}
```
