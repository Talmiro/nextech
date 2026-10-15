# Encoding

The `Encoding` struct represents the encoding of a payload in Zenoh. It describes the format of the data being transmitted.

## Built-in Encodings

Zenoh provides many built-in encodings:

```rust
use zenoh::bytes::Encoding;

// Common text encodings
Encoding::TEXT_PLAIN       // Plain text
Encoding::APPLICATION_JSON // JSON
Encoding::TEXT_XML         // XML
Encoding::TEXT_HTML        // HTML
Encoding::TEXT_CSS         // CSS
Encoding::TEXT_JAVASCRIPT  // JavaScript
Encoding::TEXT_MARKDOWN    // Markdown

// Binary encodings
Encoding::APPLICATION_OCTET_STREAM // Raw bytes
Encoding::APPLICATION_PROTOBUF     // Protocol Buffers
Encoding::APPLICATION_CBOR          // CBOR
Encoding::APPLICATION_MESSAGE_PACK   // MessagePack

// Serialization formats
Encoding::APPLICATION_SERIALIZE_MSG_PACK
Encoding::APPLICATION_SERIALIZE_JSON
Encoding::APPLICATION_SERIALIZE_YAML

// Domain-specific
Encoding::IMAGE_PNG
Encoding::IMAGE_JPEG
Encoding::VIDEO_MP4
Encoding::AUDIO_WAV

// Custom
Encoding::custom("application/my-custom-type")
```

## Using Encoding with Publish

```rust
use zenoh::bytes::Encoding;

// JSON encoding
session.put("demo/json", r#"{"key": "value"}"#)
    .encoding(Encoding::APPLICATION_JSON)
    .await
    .unwrap();

// Plain text
session.put("demo/text", "Hello world")
    .encoding(Encoding::TEXT_PLAIN)
    .await
    .unwrap();

// Binary data
let bytes: Vec<u8> = vec![0x00, 0x01, 0x02];
session.put("demo/binary", bytes)
    .encoding(Encoding::APPLICATION_OCTET_STREAM)
    .await
    .unwrap();
```

## Reading Encoding from Sample

```rust
let subscriber = session.declare_subscriber("demo/**").await.unwrap();

while let Ok(sample) = subscriber.recv_async().await {
    let encoding = sample.encoding();
    let payload = sample.payload();
    
    // Check encoding type
    if encoding == Encoding::APPLICATION_JSON {
        // Parse as JSON
    } else if encoding == Encoding::TEXT_PLAIN {
        // Parse as text
    }
}
```

## Custom Encodings

```rust
// Create custom encoding
let custom = Encoding::custom("application/x-my-format");

// Use custom encoding
session.put("demo/custom", data)
    .encoding(custom)
    .await
    .unwrap();
```

## Common Patterns

### JSON API

```rust
#[derive(Serialize, Deserialize)]
struct Data {
    temperature: f32,
    humidity: f32,
}

let data = Data { temperature: 25.5, humidity: 60.0 };
let json = serde_json::to_string(&data).unwrap();

session.put("sensors/room1", json)
    .encoding(Encoding::APPLICATION_JSON)
    .await
    .unwrap();
```

### Sensor Data (CBOR)

```rust
let payload = cbor::encode(&sensor_reading).unwrap();

session.put("sensors/temp", payload)
    .encoding(Encoding::APPLICATION_CBOR)
    .await
    .unwrap();
```

## See Also

- [Session](session.md) - Publishing and subscribing
- [ZBytes](zbytes.md) - Data payload handling
- [Pub/Sub](pubsub.md) - More pub/sub examples
