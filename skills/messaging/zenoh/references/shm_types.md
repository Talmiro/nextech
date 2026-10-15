# SHM Types (Shared Memory)

Requires `shared-memory` feature flag.

## ShmProvider

Main shared memory provider.

```rust
let provider = ShmProvider::builder().await.unwrap();
let mut shm: ZShmMut = provider.alloc(1024).await.unwrap();
```

## ShmProviderBuilder

Builder for ShmProvider.

```rust
let provider = ShmProvider::builder()
    .size(1024 * 1024)
    .backend(PosixShmProviderBackend::builder())
    .await
    .unwrap();
```

## ShmClientStorage

Storage for SHM clients.

```rust
let storage = ShmClientStorage::builder()
    .add_client(ShmProvider::default())
    .unwrap();
```

## ZShm / ZShmMut

Shared memory buffer types.

```rust
pub struct ZShm { /* private fields */ }  // Immutable
pub struct ZShmMut { /* private fields */ }  // Mutable
```

## zshm / zshmmut

Wrapper types for SHM data.

## Memory Layout

### MemoryLayout

Describes a memory layout.

### PrecomputedLayout

Precomputed memory layout.

### TypedLayout

Typed memory layout.

### Typed

Typed wrapper.

## Allocation

### AllocBuilder

Builder for allocations.

### AllocatedChunk

Allocated memory chunk.

### ChunkDescriptor

Descriptor for a chunk.

### PtrInSegment

Pointer in shared memory segment.

### AllocAlignment

Alignment for allocations.

### ConstBool / ConstUsize

Compile-time constants.

## Backend Types

### PosixShmProviderBackend

POSIX shared memory backend.

### PosixShmProviderBackendBinaryHeap

Binary heap backend.

### PosixShmProviderBackendBuddy

Buddy system backend.

### PosixShmProviderBackendTalc

Talc allocator backend.

## Policies

### BlockOn

Block on allocation.

### JustAlloc

Just allocate.

### Deallocate

Deallocation policy.

### Defragment

Defragmentation policy.

### GarbageCollect

Garbage collection policy.

## Error Types

### ZAllocError

Allocation error.

### ZLayoutError

Layout error.

### ZLayoutAllocError

Layout allocation error.

### BufferRelayoutError

Buffer relayout error.

### ShmProviderState

Provider state enum.

## Constants

### GLOBAL_CLIENT_STORAGE

Global SHM client storage.

## Type Aliases

### ChunkAllocResult

`Result<AllocatedChunk, ZAllocError>`

### ChunkID

Chunk identifier.

### ProtocolID

Protocol identifier.

### SegmentID

Segment identifier.

### PosixShmProviderBackend

`PosixShmProviderBackendBinaryHeap`

### PosixShmProviderBackendBuilder

`PosixShmProviderBackendBinaryHeapBuilder`
