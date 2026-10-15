# Query/Reply Types: Querier, Queryable, Selector, KeyExpr

## Querier

A querier sends queries to Queryables. Pre-configured object for sending multiple queries.

```rust
let session = zenoh::open(zenoh::Config::default()).await.unwrap();
let querier = session.declare_querier("key/expression").await.unwrap();
let replies = querier.get().await.unwrap();
```

**Key Methods:**
- `get()` - Send a query, returns builder for replies
- `key_expr()` - Get the key expression
- `congestion_control()` - Get congestion control setting
- `priority()` - Get priority
- `matching_status()` - Check if queryables exist
- `matching_listener()` - Listen for matching status changes
- `undeclare()` - Undeclare the querier

---

## QuerierBuilder

Builder for initializing a Querier. Returned by `Session::declare_querier()`.

**Builder Methods:**
- `encoding(encoding)` - Set encoding
- `congestion_control(control)` - Set congestion control
- `priority(priority)` - Set priority
- `target(target)` - Set query target
- `consolidation(mode)` - Set consolidation mode
- `timeout(timeout)` - Set timeout

---

## Queryable<Handler>

An entity that implements query/reply pattern. Receives queries and sends replies.

**Callback style:**
```rust
let queryable = session
    .declare_queryable("key/expression")
    .callback(move |query| {
        query.reply("key/expression", "value").wait().unwrap();
    })
    .await
    .unwrap();
```

**Channel style:**
```rust
let queryable = session.declare_queryable("key/expression").await.unwrap();
while let Ok(query) = queryable.recv_async().await {
    query.reply("key/expression", "value").await.unwrap();
}
```

**Key Methods:**
- `key_expr()` - Get the key expression
- `handler()` - Get handler reference
- `handler_mut()` - Get mutable handler reference
- `undeclare()` - Undeclare the queryable

---

## Query

Received query with methods to send replies.

**Key Methods:**
- `selector()` - Get the selector (key expression + parameters)
- `parameters()` - Get query parameters
- `reply(key_expr, payload)` - Send a reply
- `reply_err(error)` - Send an error reply
- `reply_del(key_expr)` - Send a delete reply

---

## Selector

Combination of KeyExpr and Parameters for queries.

```rust
// Format: key_expr?params
let selector: Selector = "demo/key?timeout=1000".parse().unwrap();
let key_expr = selector.key_expr();
let params = selector.parameters();
```

**Methods:**
- `key_expr()` - Get the key expression
- `parameters()` - Get the parameters
- `split()` - Deconstruct into (KeyExpr, Parameters)

---

## KeyExpr

Key expressions define the set of keys for operations. Support wildcards and set intersection.

```rust
// Exact key
let ke = KeyExpr::try_from("device/sensor/temperature").unwrap();

// Wildcard matching multiple keys
let ke = KeyExpr::try_from("device/sensor/*").unwrap();

// Set intersection - matches multiple paths
let ke1 = KeyExpr::try_from("device/**").unwrap();
let ke2 = KeyExpr::try_from("*/sensor/*").unwrap();
assert!(ke1.intersects(&ke2));
```

**Creation:**
- `KeyExpr::try_from("string")` - Validate and create (must be canonical)
- `KeyExpr::autocanonize("string")` - Auto-canonize and create
- `unsafe fn from_str_unchecked("string")` - Create without validation

**Methods:**
- `key_expr()` - Get borrowed version
- `into_owned()` - Ensure ownership
- `join(&suffix)` - Join with separator
- `concat(&suffix)` - Concatenate strings

**Deref to keyexpr with:**
- `intersects(&other)` - Check if sets intersect
- `includes(&other)` - Check if includes other
- `relation_to(&other)` - Get set relationship
- `as_str()` - Get string representation

**Operators:**
- `keyexpr / "suffix"` - Join with `/` separator (infallible)

---

## Parameters

Query parameters (after `?` in selector).

```rust
let params: Parameters = selector.parameters();
// Common params:
//   _time - time range (unstable)
//   _anyke - accept any key expression (unstable)
```

---

## Query Consolidation

Controls how replies are consolidated.

### ConsolidationMode
- `None` - No consolidation
- `Monotonic` - Latest value wins
- `Latest` - Latest complete sample
- `Oldest` - Oldest complete sample

---

## Query Target

Target for queries.

### QueryTarget
- `Any` - Any matching queryable
- `All` - All matching queryables  
- `BestMatching` - Best matching queryable
