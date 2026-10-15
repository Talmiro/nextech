# Key Expressions

Zenoh uses hierarchical key expressions (like filesystem paths) for pub/sub filtering.

## Basic Key Expressions

```rust
use zenoh::prelude::*;

// Exact match
let key = keyexpr::new("demo/device/sensor1")?;

// Wildcard matching
let key = keyexpr::new("demo/device/*")?;     // Single level wildcard
let key = keyexpr::new("demo/**")?;           // Multi-level wildcard
let key = keyexpr::new("demo/device/##")?;    // Tail wildcard
```

## Key Expression Rules

- `/` separates path segments
- `*` matches single segment (any characters except `/`)
- `**` matches multiple segments
- `##` matches tail (zero or more segments at end)

## Examples

| Key Expression | Matches | Doesn't Match |
|----------------|---------|---------------|
| `a/b/*` | `a/b/c` | `a/b` |
| `a/b/**` | `a/b/c`, `a/b/c/d` | `a/c` |
| `a/*/c` | `a/b/c` | `a/b/d/c` |
| `a##` | `a`, `a/b`, `a/b/c` | `b` |

## Using Key Expressions

```rust
use zenoh::prelude::*;

let session = zenoh::open(zenoh::config::Config::default()).await?;

// Subscribe with wildcard
let subscriber = session.declare_subscriber("demo/device/*").await?;

// Publish to specific key
let publisher = session.declare_publisher("demo/device/sensor1").await?;
publisher.put("data").await?;

// Check key intersection
let ke1 = keyexpr::new("demo/**")?;
let ke2 = keyexpr::new("demo/device/sensor1")?;

if ke1.intersects(&ke2) {
    println!("Keys intersect!");
}
```

## Key Expression Canonization

```rust
let key = keyexpr::new("demo//device///sensor1")?;
let canonical = key.canonize()?;  // "demo/device/sensor1"
```

## See Also

- [Pub/Sub](pubsub.md) — Using key expressions in pub/sub
- [Query/Reply](reqrep.md) — Using key expressions in queries
