# Scout, Hello, QoS, and Time Types

## Scout

`Scout` - A scout that returns `Hello` messages through a Handler.

```rust
use zenoh::config::WhatAmI;

let receiver = zenoh::scout(WhatAmI::Peer | WhatAmI::Router, zenoh::Config::default())
    .with(flume::bounded(32))
    .await
    .unwrap();
while let Ok(hello) = receiver.recv_async().await {
    println!("{}", hello);
}
```

### Methods

- `stop(self)` - Stop scouting

```rust
let scout = zenoh::scout(WhatAmI::Router, zenoh::Config::default())
    .with(flume::bounded(32))
    .await
    .unwrap();
let _router = scout.recv_async().await;
scout.stop();
```

## Hello

`Hello` - A zenoh Hello message received during scouting.

### Methods

- `locators(&self) -> &[Locator]` - Get the locators of this Hello message
- `zid(&self) -> ZenohId` - Get the zenoh id of this Hello message
- `whatami(&self) -> WhatAmI` - Get the whatami of this Hello message

## QoS (Quality of Service)

### Priority

`Priority` - Message priority levels (1=RealTime to 7=Background)

```rust
#[repr(u8)]
pub enum Priority {
    RealTime = 1,        // Highest priority
    InteractiveHigh = 2,
    InteractiveLow = 3,
    DataHigh = 4,
    Data = 5,            // Default
    DataLow = 6,
    Background = 7,      // Lowest priority
}
```

Used in PublicationBuilder and SubscriberBuilder for QoS control.

### CongestionControl

`CongestionControl` - Flow control for publishers

```rust
pub enum CongestionControl {
    Block,   // Block when queue is full
    Drop,    // Drop messages when queue is full (default)
}
```

### Reliability

`Reliability` - Message delivery reliability

```rust
pub enum Reliability {
    Reliable,   // Guaranteed delivery
    BestEffort, // Best effort delivery
}
```

## Time

### NTP64

`NTP64` - NTP 64-bit timestamp format as specified in RFC-5909

```rust
pub struct NTP64(pub u64);
```

### Timestamp

`Timestamp` - A timestamp made of a NTP64 time and a TimestampId

```rust
pub struct Timestamp { /* private fields */ }
```

Each Sample has an optional Timestamp indicating when it was generated.

### TimestampId

`TimestampId` - Type alias for TimestampId (from uhlc crate)

```rust
type TimestampId = uhlc::TimestampId;
```
