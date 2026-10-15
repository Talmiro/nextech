# Sample, Reply

Core data types for receiving data in Zenoh.

## Sample

The `Sample` struct represents data received from a subscriber or querier.

### Receiving Samples

```rust
use zenoh::prelude::*;

let subscriber = session.declare_subscriber("demo/**").await.unwrap();

// Receive a sample
let sample: Sample = subscriber.recv_async().await.unwrap();

// Access sample data
let key_expr = sample.key_expr();
let payload = sample.payload();
let encoding = sample.encoding();
let kind = sample.kind();  // Put or Delete

// Get timestamp
if let Some(ts) = sample.timestamp() {
    println!("Timestamp: {}", ts);
}
```

### Sample Methods

```rust
let sample: Sample = subscriber.recv_async().await.unwrap();

// Key expression
let key: &KeyExpr = sample.key_expr();

// Payload as ZBytes
let data: &ZBytes = sample.payload();

// Encoding
let enc: &Encoding = sample.encoding();

// Sample kind (Put or Delete)
let kind: SampleKind = sample.kind();

// Timestamp (if available)
let ts: Option<&Timestamp> = sample.timestamp();

// QoS properties
let priority: Priority = sample.priority();
let congestion: CongestionControl = sample.congestion_control();
let express: bool = sample.express();

// Attachment (custom metadata)
let attachment: Option<&ZBytes> = sample.attachment();
```

### SampleKind

```rust
use zenoh::sample::SampleKind;

match sample.kind() {
    SampleKind::Put => println!("Data was put"),
    SampleKind::Delete => println!("Data was deleted"),
}
```

## Reply

The `Reply` is received when querying data.

### Receiving Replies

```rust
use zenoh::prelude::*;

// Using Session::get
let replies = session.get("demo/**").await.unwrap();

while let Ok(reply) = replies.recv_async().await {
    match reply.result() {
        Ok(sample) => {
            println!("Reply: {:?}", sample);
        }
        Err(e) => {
            eprintln!("Reply error: {:?}", e);
        }
    }
}
```

### Reply API

```rust
let replies = session.get("demo/**").await.unwrap();

while let Ok(reply) = replies.recv_async().await {
    // Check if reply is successful
    if reply.is_ok() {
        let sample: Sample = reply.into_ok().unwrap();
        println!("Got: {:?}", sample);
    } else {
        let error = reply.into_err().unwrap();
        eprintln!("Error: {:?}", error);
    }
    
    // Or use result() directly
    match reply.result() {
        Ok(sample) => println!("Reply: {:?}", sample),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### ReplyErr

Error information in a reply:

```rust
use zenoh::query::ReplyErr;

let replies = session.get("demo/**").await.unwrap();

while let Ok(reply) = replies.recv_async().await {
    if let Err(reply_err) = reply.result() {
        // ReplyErr contains error info
        println!("Error code: {:?}", reply_err.code());
        println!("Error message: {:?}", reply_err.payload());
    }
}
```

## Complete Examples

### Subscriber

```rust
use zenoh::prelude::*;

let subscriber = session.declare_subscriber("sensors/**").await.unwrap();

loop {
    match subscriber.recv_async().await {
        Ok(sample) => {
            let key = sample.key_expr();
            let payload = sample.payload();
            
            // Parse based on encoding
            if sample.encoding() == &Encoding::APPLICATION_JSON {
                if let Ok(s) = std::str::from_utf8(payload) {
                    println!("{}: {}", key, s);
                }
            }
        }
        Err(e) => {
            eprintln!("Receive error: {:?}", e);
            break;
        }
    }
}
```

### Querier

```rust
use zenoh::prelude::*;

let querier = session.declare_querier("demo/service").await.unwrap();
let replies = querier.get()
    .target(WhatAmI::Peer)
    .await
    .unwrap();

let mut count = 0;
while let Ok(reply) = replies.recv_async().await {
    if let Ok(sample) = reply.result() {
        count += 1;
        println!("Reply {} from {}: {:?}", 
            count, 
            sample.key_expr(), 
            sample.payload()
        );
    }
}
```

## See Also

- [Session](session.md) - Publishing and subscribing
- [Pub/Sub](pubsub.md) - Pub/Sub patterns
- [Query/Reply](reqrep.md) - Query patterns
- [ZBytes](zbytes.md) - Working with payloads
- [Encoding](encoding.md) - Data encoding
