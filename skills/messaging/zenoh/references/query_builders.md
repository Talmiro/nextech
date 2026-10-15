# Query/Reply Builders

## QuerierBuilder

Builder to create a querier.

```rust
let querier = session.declare_querier("demo/key")
    .target(QueryTarget::All)
    .consolidation(QueryConsolidation::latest())
    .timeout(Duration::from_secs(5))
    .await
    .unwrap();
```

### Methods

- `target(self, target: QueryTarget) -> Self`
- `consolidation(self, consolidation: QueryConsolidation) -> Self`
- `timeout(self, timeout: Duration) -> Self`
- `locality(self, locality: Locality) -> Self`

## QuerierGetBuilder

Builder for get operations on a querier.

```rust
let replies = querier.get().await?;
```

## QueryableBuilder

Builder to declare a queryable.

```rust
let queryable = session.declare_queryable("demo/key")
    .target(QueryTarget::AllMatchable)
    .complete(true)
    .await
    .unwrap();
```

### Methods

- `target(self, target: QueryTarget) -> Self`
- `complete(self, complete: bool) -> Self` - Is this a complete source?

## ReplyBuilder

Builder to send a reply to a query.

```rust
query.reply("OK").payload("data").await?;
```

### Variants

- `ReplyBuilderPut` - Send a put reply
- `ReplyBuilderDelete` - Send a delete reply
- `ReplyErrBuilder` - Send an error reply

## Query Types

### Query

A query received by a queryable.

```rust
fn handle_query(query: Query) {
    let key_expr = query.key_expr();
    let selector = query.selector();
    let parameters = query.parameters();
    
    // Send reply
    query.reply("OK").payload("data").await;
}
```

### Parameters

Query parameters (key-value pairs).

```rust
let params = query.parameters();
let value = params.get("key");
```

### Selector

```rust
pub struct Selector {
    /* private fields */
}
```

### TimeRange

```rust
pub struct TimeRange {
    /* private fields */
}
```

### QueryConsolidation

```rust
pub struct QueryConsolidation {
    pub mode: ConsolidationMode,
    pub any_origin: bool,
}
```

### ConsolidationMode

```rust
pub enum ConsolidationMode {
    None,           // No consolidation
    Monotone,       // Last reply wins
    Full,           // Complete history
}
```

### QueryTarget

```rust
pub enum QueryTarget {
    BestMatch,
    All,
    AllMatchable,
}
```

### ReplyKeyExpr

```rust
pub enum ReplyKeyExpr {
    MatchAll,
    Some(KeyExpr),
}
```

### TimeBound / TimeExpr

Time-based query options.
