# Traits

## Resolvable

Trait for types that can be resolved to a value.

```rust
pub trait Resolvable {
    type To;
    fn resync(self) -> Self::To;
}
```

All builder types implement `Resolvable` (e.g., `OpenBuilder`, `PublisherBuilder`, etc.)

## Wait

Synchronous version of Resolvable.

```rust
pub trait Wait {
    fn wait(self) -> Self::To;
}
```

## Resolve

Trait for resolving async operations.

```rust
pub trait Resolve<T> {
    type Output;
    fn resolve(self) -> Self::Output;
}
```

## IntoHandler

Trait for converting types into handlers.

```rust
pub trait IntoHandler<T, H> {
    fn into_handler(self) -> H;
}
```

## Canonize

Trait for key expression canonization.

```rust
pub trait Canonize {
    fn canonize(self) -> Result<OwnedKeyExpr, KeyExprError>;
    fn canonize_in_place(&mut self) -> Result<(), KeyExprError>;
}
```

## Undeclarable

Trait for undeclaring entities (publishers, subscribers, etc.).

```rust
pub trait Undeclarable<Undeclare> {
    fn undeclare(self) -> Undeclare;
}
```

## ZenohParameters

Trait for types that can have parameters.

```rust
pub trait ZenohParameters {
    fn with_parameters(self, params: &str) -> Self;
}
```

# SHM Traits (shared-memory)

## ShmBuf

Trait for shared memory buffers.

## ShmBufMut

Mutable shared memory buffer.

## ShmBufUnsafeMut

Unsafe mutable shared memory buffer.

## ShmBufIntoImmut

Convert mutable SHM buffer to immutable.

## OwnedShmBuf

Owned shared memory buffer.

## ShmSegment

Shared memory segment.

## ShmClient

Client for shared memory operations.

## ShmProviderBackend

Backend for shared memory provider.

## AllocPolicy

Allocation policy for shared memory.

## AsyncAllocPolicy

Async allocation policy.

## ConstPolicy

Constant allocation policy.

## PolicyValue

Value for allocation policy.

## ResideInShm

Types that can reside in shared memory.

## SafePolicy

Safe allocation policy.

## WithProtocolID

Types with protocol ID.

### IKeFormatStorage

Key expression format storage trait.
