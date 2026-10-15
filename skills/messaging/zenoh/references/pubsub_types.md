# Pub/Sub Types: Publisher, Subscriber, Builders

## Publisher

A publisher that allows sending data through a stream. Publishers are automatically undeclared when dropped.

```rust
let session = zenoh::open(zenoh::Config::default()).await.unwrap();
let publisher = session.declare_publisher("key/expression").await.unwrap();
publisher.put("value").await.unwrap();
```

**Key Methods:**
- `put(payload)` - Publish data (SampleKind::Put)
- `delete()` - Publish deletion (SampleKind::Delete)
- `key_expr()` - Get the key expression
- `encoding()` - Get the encoding
- `congestion_control()` - Get congestion control setting
- `priority()` - Get priority
- `matching_status()` - Check if subscribers exist
- `matching_listener()` - Listen for matching status changes
- `undeclare()` - Explicitly undeclare the publisher

**Implements:** `Sink<Sample>` for forwarding streams

---

## PublisherBuilder

Builder for initializing a Publisher. Returned by `Session::declare_publisher()`.

```rust
let publisher = session
    .declare_publisher("key/expression")
    .congestion_control(CongestionControl::Block)
    .priority(Priority::RealTime)
    .await
    .unwrap();
```

**Builder Methods:**
- `encoding(encoding)` - Set default encoding
- `congestion_control(control)` - Set congestion control
- `priority(priority)` - Set priority
- `express(is_express)` - Enable express mode (no batching)
- `allowed_destination(locality)` - Restrict to local subscribers
- `reliability(reliability)` - Set reliability (unstable)

---

## Subscriber<Handler>

A subscriber that provides data through a handler. Subscribers can use callbacks or channel handlers.

**Callback style:**
```rust
session
    .declare_subscriber("key/expression")
    .callback(|sample| { println!("Received: {:?}", sample); })
    .background()
    .await
    .unwrap();
```

**Channel style:**
```rust
let subscriber = session
    .declare_subscriber("key/expression")
    .with(flume::bounded(32))
    .await
    .unwrap();
while let Ok(sample) = subscriber.recv_async().await {
    println!("Received: {:?}", sample);
}
```

**Key Methods:**
- `key_expr()` - Get subscribed key expression
- `handler()` - Get reference to handler
- `handler_mut()` - Get mutable reference to handler
- `undeclare()` - Undeclare the subscriber

**Implements:** `Deref<Target = Handler>` for direct handler access

---

## SubscriberBuilder

Builder for initializing a Subscriber. Returned by `Session::declare_subscriber()`.

```rust
let subscriber = session
    .declare_subscriber("key/expression")
    .callback(|sample| { println!("Received: {:?}", sample); })
    .await
    .unwrap();
```

**Builder Methods:**
- `callback(callback)` - Set callback handler (Fn)
- `callback_mut(callback)` - Set mutable callback (FnMut)
- `with(handler)` - Set custom handler (IntoHandler)
- `background()` - Run in background (no Subscriber returned)
- `allowed_origin(locality)` - Restrict to local publishers

---

## QoS Types (used by Publisher/Subscriber)

### Priority
- `RealTime`, `Interactive`, `DataHigh`, `Data`, `DataLow`, `Background`

### CongestionControl  
- `Block` - Block when queue full
- `Drop` - Drop when queue full

### Reliability
- `Reliable` - Guarantee delivery
- `BestEffort` - No guarantee

### Locality
- `Any` - Any publisher
- `SessionLocal` - Same session only
- `PeerLocal` - Same peer only
- `Remote` - Remote only
