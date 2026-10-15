# Handlers

Handlers in Zenoh are used to receive data from subscribers and queryables.

## DefaultHandler

The default handler type, wrapping a FIFO channel with default settings.

```rust
use zenoh::handlers::DefaultHandler;

// Default handler is used when no handler is specified
let subscriber = session
    .declare_subscriber("demo/**")
    .await
    .unwrap();

// Receive using default handler
let sample = subscriber.recv_async().await.unwrap();
```

## FifoChannel

Customizable FIFO channel handler:

```rust
use zenoh::handlers::FifoChannel;

let (tx, rx) = FifoChannel::new(16);  // capacity of 16

let subscriber = session
    .declare_subscriber("demo/**")
    .handler(tx)
    .await
    .unwrap();

// Receive from channel
while let Ok(sample) = rx.recv_async().await {
    println!("Got: {:?}", sample);
}
```

## Callback

Use callbacks for event-driven receiving:

```rust
use zenoh::handlers::Callback;

// Callback handler
let callback = Callback::<Sample>::new();

let subscriber = session
    .declare_subscriber("demo/**")
    .handler(callback.clone())
    .await
    .unwrap();

// Set the callback
callback.set(async move |sample| {
    println!("Received: {:?}", sample);
});
```

## CallbackMut

Mutable callback for stateful handlers:

```rust
use zenoh::handlers::CallbackMut;

let mut counter = 0;
let callback = CallbackMut::new(move |sample| {
    counter += 1;
    println!("Received {}: {:?}", counter, sample);
});

let subscriber = session
    .declare_subscriber("demo/**")
    .handler(callback)
    .await
    .unwrap();
```

## IntoHandler Trait

Convert various types into handlers:

```rust
use zenoh::handlers::IntoHandler;
use zenoh::prelude::*;

// From receiver
let receiver = async_channel::bounded(10);
let handler = receiver.0.into_handler();

// From callback
let callback = Callback::<Sample>::new();
let handler = callback.into_handler();

// Default
let handler = DefaultHandler::default();
```

## Handler Types Summary

| Handler | Use Case |
|---------|----------|
| DefaultHandler | Simple recv_async() |
| FifoChannel | Bounded channel with capacity |
| Callback | Event-driven, set later |
| CallbackMut | Stateful callbacks |

## Complete Example

```rust
use zenoh::prelude::*;
use zenoh::handlers::{FifoChannel, Callback};

#[tokio::main]
async fn main() {
    let session = zenoh::open(Config::default()).await.unwrap();

    // Using callback
    let callback = Callback::<Sample>::new();
    let sub = session
        .declare_subscriber("demo/**")
        .handler(callback.clone())
        .await
        .unwrap();

    callback.set(|sample| {
        println!("Callback got: {:?}", sample);
    }).unwrap();

    // Or using fifo channel
    let (tx, rx) = FifoChannel::new(16);
    let _sub2 = session
        .declare_subscriber("demo/**")
        .handler(tx)
        .await
        .unwrap();

    // Receive from channel
    while let Ok(sample) = rx.recv_async().await {
        println!("Channel got: {:?}", sample);
    }
}
```

## See Also

- [Session](session.md) - Subscribers and queryables
- [Pub/Sub](pubsub.md) - Publishing and subscribing
- [Query/Reply](reqrep.md) - Query patterns
