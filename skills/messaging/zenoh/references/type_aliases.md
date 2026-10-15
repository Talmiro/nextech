# Type Aliases

This file documents all type aliases in the zenoh crate.

## Core Types

### Error

```rust
pub type Error = Box<dyn Error + Send + Sync>;
```

The main error type in zenoh.

### Result

```rust
pub type Result<T, E = Error> = Result<T, E>;
```

The main result type in zenoh.

## Pub/Sub Type Aliases

### PublisherPutBuilder

```rust
pub type PublisherPutBuilder<'a, 'b> = PublicationBuilder<PublisherBuilder<'a, 'b>, PublicationBuilderPut>;
```

Builder for publishing put operations.

### PublisherDeleteBuilder

```rust
pub type PublisherDeleteBuilder<'a, 'b> = PublicationBuilder<PublisherBuilder<'a, 'b>, PublicationBuilderDelete>;
```

Builder for publishing delete operations.

## Session Type Aliases

### EntityId

```rust
pub type EntityId = u32;
```

Local entity identifier within a session.

### SessionPutBuilder

```rust
pub type SessionPutBuilder<'a, 'b> = PublicationBuilder<PublisherBuilder<'a, 'b>, PublicationBuilderPut>;
```

Session-level put builder.

### SessionDeleteBuilder

```rust
pub type SessionDeleteBuilder<'a, 'b> = PublicationBuilder<PublisherBuilder<'a, 'b>, PublicationBuilderDelete>;
```

Session-level delete builder.

## Sample Type Aliases

### SourceSn

```rust
pub type SourceSn = u32;
```

Source sequence number.

## Time Type Aliases

### TimestampId

```rust
pub type TimestampId = uhlc::TimestampId;
```

Timestamp identifier (from uhlc crate).

## SHM Type Aliases

### ChunkAllocResult

```rust
pub type ChunkAllocResult = Result<AllocatedChunk, ZAllocError>;
```

Result of chunk allocation.

### ChunkID

```rust
pub type ChunkID = u64;
```

Chunk identifier.

### ProtocolID

```rust
pub type ProtocolID = u8;
```

Protocol identifier for SHM.

### SegmentID

```rust
pub type SegmentID = u32;
```

Segment identifier.

### PosixShmProviderBackend

```rust
pub type PosixShmProviderBackend = PosixShmProviderBackendBinaryHeap;
```

Default POSIX SHM provider backend.

### PosixShmProviderBackendBuilder

```rust
pub type PosixShmProviderBackendBuilder = PosixShmProviderBackendBinaryHeapBuilder;
```

Builder for POSIX SHM provider backend.
