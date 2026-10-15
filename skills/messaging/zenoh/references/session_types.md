# Session Types

## Session Builders

### OpenBuilder

A builder returned by `zenoh::open()` used to open a zenoh Session.

```rust
let session = zenoh::open(zenoh::Config::default()).await.unwrap();
```

#### Methods

- `with_shm_clients(self, shm_clients: Arc<ShmClientStorage>) -> Self` (requires `shared-memory` feature)

#### Trait Implementations

- `Resolvable` - Returns `Result<Session, Box<dyn Error + Sync + Send>>`
- `Wait` - Synchronously execute and wait
- `IntoFuture` - Can be used with `.await`

### CloseBuilder

A builder to close a zenoh Session.

```rust
session.close().await.unwrap();
```

## Session Info and Statistics

### SessionInfo

Provides access to session information and statistics.

```rust
let info = session.info();
```

### ZenohId

The unique identifier of the zenoh node.

```rust
let zid: ZenohId = session.zid();
```

### ZenohIdBuilder

Builder to create a ZenohId.

## Transport and Links

### Transport

Represents a transport (connection) to another zenoh node.

### Link

Represents a single link (connection) within a transport.

```rust
pub struct Link { /* private fields */ }
```

### LinkEvent

Events from a link (opened, closed, etc.)

### LinkEvent / TransportEvent

- `LinkEventsListener` - Listen to link events
- `TransportEventsListener` - Listen to transport events

### LinksBuilder

Builder to get all links of a session.

```rust
let links = session.links().await?;
```

### TransportsBuilder

Builder to get all transports of a session.

## Entity Identity

### EntityGlobalId

Global identifier for a zenoh entity (publisher, subscriber, etc.)

```rust
pub struct EntityGlobalId {
    pub zid: ZenohId,
    pub eid: EntityId,
}
```

### EntityId

Local entity identifier within a session.

## Session Error Types

### SessionClosedError

Error returned when trying to use a closed session.

```rust
pub struct SessionClosedError;
```

## Type Aliases

### SessionPutBuilder

Type alias for `PublicationBuilder<PublisherBuilder<'a, 'b>, PublicationBuilderPut>`

### SessionDeleteBuilder

Type alias for `PublicationBuilder<PublisherBuilder<'a, 'b>, PublicationBuilderDelete>`

### SessionGetBuilder

Type alias for `query::QuerierGetBuilder`

### EntityId

Type alias for `u32`
