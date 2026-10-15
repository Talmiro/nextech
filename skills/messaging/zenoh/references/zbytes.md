# ZBytes

`ZBytes` is the primary data type in Zenoh for transmitting payloads. It's similar to `bytes::Bytes` but optimized for Zenoh's protocol.

## Creating ZBytes

```rust
use zenoh::bytes::*;

// From string
let zb = ZBytes::from("hello world");

// From Vec<u8>
let zb = ZBytes::from(vec![0u8, 1, 2, 3]);

// From String
let s = String::from("test");
let zb = ZBytes::from(s);

// From slices
let data: &[u8] = &[1, 2, 3];
let zb = ZBytes::from(data);

// Using Into trait
let zb: ZBytes = "string".into();
let zb: ZBytes = vec![1, 2, 3].into();
```

## Working with ZBytes

```rust
let zb = ZBytes::from("test data");

// Length
let len = zb.len();

// Check empty
if !zb.is_empty() {
    println!("Has {} bytes", len);
}

// As slice
let slice: &[u8] = &zb;
```

## ZBytesReader

For reading large data in chunks:

```rust
use zenoh::bytes::ZBytesReader;

let zb = ZBytes::from("large data content...");
let mut reader = zb.reader();

// Read all
let all = reader.read_to_bytes().await.unwrap();

// Read chunk by chunk
let mut reader = zb.reader();
let mut buffer = vec![0u8; 1024];
while let Ok(n) = reader.read(&mut buffer).await {
    if n == 0 { break; }
    let chunk = &buffer[..n];
    println!("Chunk: {:?}", chunk);
}
```

## ZBytesWriter

For writing data incrementally:

```rust
use zenoh::bytes::ZBytesWriter;

let mut writer = ZBytesWriter::new();

writer.write_all(b"part1").await.unwrap();
writer.write_all(b"part2").await.unwrap();
writer.write_all(b"part3").await.unwrap();

let zb = writer.into_zbytes();
```

## OptionZBytes

Optional ZBytes (can be None):

```rust
use zenoh::bytes::OptionZBytes;

// Some value
let opt: OptionZBytes = Some(ZBytes::from("data")).into();

// None
let opt: OptionZBytes = None.into();

// Check
if let Some(zb) = opt.as_ref() {
    println!("Got: {:?}", zb);
}

// Take value
if let Some(zb) = opt.take() {
    // zb is moved out
}
```

## Iterating Slices

```rust
let zb = ZBytes::from(vec![1, 2, 3, 4, 5]);

// Iterator over slices
let mut iter = zb.slice_iter();
while let Some(slice) = iter.next() {
    println!("Slice: {:?}", slice);
}
```

## With Serialization

Using serde with ZBytes:

```rust
use zenoh::bytes::ZBytes;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    value: i32,
    name: String,
}

let data = MyData { value: 42, name: "test".to_string() };

// Serialize to ZBytes
let zb = ZBytes::from(serde_json::to_vec(&data).unwrap());

// Or use IntoZBytes trait
let zb: ZBytes = data.into();

// Deserialize
let decoded: MyData = serde_json::from_slice(&zb).unwrap();
```

## Common Patterns

### Publishing Structured Data

```rust
use zenoh::bytes::*;

// JSON
let json = serde_json::to_vec(&my_struct).unwrap();
session.put("key", json)
    .encoding(Encoding::APPLICATION_JSON)
    .await
    .unwrap();

// Custom binary format
let bytes = my_binary_data();
session.put("key", bytes).await.unwrap();
```

### Receiving Data

```rust
let subscriber = session.declare_subscriber("demo/**").await.unwrap();

while let Ok(sample) = subscriber.recv_async().await {
    let zb = sample.payload();
    
    // Convert to string
    if let Ok(s) = std::str::from_utf8(&zb) {
        println!("Text: {}", s);
    }
    
    // Get raw bytes
    let bytes: &[u8] = &zb;
}
```

## See Also

- [Session](session.md) - Publishing and subscribing
- [Encoding](encoding.md) - Data encoding
- [Pub/Sub](pubsub.md) - More examples
