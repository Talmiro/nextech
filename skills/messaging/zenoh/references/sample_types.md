# Sample Types

## Sample

`Sample` - A data sample received from zenoh (publish or query reply).

```rust
use zenoh::prelude::*;

let subscriber = session.declare_subscriber("demo/key").await.unwrap();
let sample: Sample = subscriber.recv_async().await.unwrap();

let key_expr = sample.key_expr();
let payload = sample.payload();
let encoding = sample.encoding();
let timestamp = sample.timestamp();
```

### Methods

- `key_expr(&self) -> &KeyExpr` - The key expression of the sample
- `payload(&self) -> &ZBytes` - The payload data
- `encoding(&self) -> &Encoding` - The encoding of the payload
- `timestamp(&self) -> Option<&Timestamp>` - Optional timestamp
- `source_info(&self) -> &SourceInfo` - Information about the source
- `kind(&self) -> SampleKind` - Put or Delete

## SampleBuilder

Builder to construct a Sample.

```rust
let sample = Sample::builder("demo/key")
    .payload("Hello")
    .encoding("text/plain")
    .timestamp(timestamp)
    .await?;
```

### Variants

- `SampleBuilderPut` - Builder for PUT samples
- `SampleBuilderDelete` - Builder for DELETE samples
- `SampleBuilderAny` - Generic sample builder

### Methods

- `key_expr<K>(self, key_expr: K) -> Self` - Set key expression
- `payload<P: Into<ZBytes>>(self, payload: P) -> Self` - Set payload
- `encoding<E: Into<Encoding>>(self, encoding: E) -> Self` - Set encoding
- `timestamp(self, timestamp: Timestamp) -> Self` - Set timestamp
- `source_info(self, source_info: SourceInfo) -> Self` - Set source info

## SampleFields

Access to sample fields with builders.

## SourceInfo

Information about the source of a sample.

```rust
pub struct SourceInfo {
    /* private fields */
}
```

### Methods

- `source_id(&self) -> Option<ZenohId>` - The source ZenohId
- `source_sn(&self) -> Option<SourceSn>` - The source sequence number

## SampleKind

```rust
pub enum SampleKind {
    Put,    // Data put operation
    Delete, // Data delete operation
}
```

## Locality

Specifies where the sample should be received from.

```rust
pub enum Locality {
    Any,            // Any location (default)
    SessionLocal,   // Same session
    ClientLocal,    // Same process
    Remote,        // Remote nodes only
}
```
