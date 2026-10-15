# Pub/Sub Builders

## PublicationBuilder

Builder for publishing data.

```rust
let publisher = session.declare_publisher("demo/key").await.unwrap();
publisher.put("Hello Zenoh!").await.unwrap();
publisher.delete().await.unwrap();
```

### Variants

- `PublicationBuilderPut` - Builder for put operations
- `PublicationBuilderDelete` - Builder for delete operations

### Methods

- `priority(self, priority: Priority) -> Self` - Set priority (1-7)
- `congestion_control(self, cc: CongestionControl) -> Self` - Set congestion control
- `express(self, express: bool) -> Self` - Express mode (no acknowledgment)
- `timestamp(self, timestamp: Timestamp) -> Self` - Set timestamp

## PublisherBuilder

Builder to declare a publisher.

```rust
let publisher = session.declare_publisher("demo/key")
    .priority(Priority::DataHigh)
    .congestion_control(CongestionControl::Block)
    .await
    .unwrap();
```

### Methods

- `priority(self, priority: Priority) -> Self`
- `congestion_control(self, cc: CongestionControl) -> Self`
- `express(self, express: bool) -> Self`
- `allowed_destination(self, destination: WhatAmI) -> Self`

## SubscriberBuilder

Builder to declare a subscriber.

```rust
let subscriber = session.declare_subscriber("demo/**")
    .reliability(BestEffort)
    .periodicity(Duration::from_secs(1))
    .await
    .unwrap();
```

### Methods

- `reliability(self, reliability: Reliability) -> Self`
- `periodicity(self, period: Duration) -> Self` - Enable late join
- `timeout(self, timeout: Duration) -> Self`
- `allowed_destination(self, destination: WhatAmI) -> Self`
- `locality(self, locality: Locality) -> Self`

## PublisherDeleteBuilder / PublisherPutBuilder

Type aliases for publication builders.
