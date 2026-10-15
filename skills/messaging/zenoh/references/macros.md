# Macros

## key_expr::format::ke

Create a key expression format at compile time.

```rust
use zenoh::key_expr::format::ke;

let format = ke!("test/{}/data");
let key_expr = format.format("device1");
```

## key_expr::format::kedefine

Define a key expression format as a static.

```rust
use zenoh::key_expr::format::kedefine;

kedefine!(MY_FORMAT, "device/{sensor}/temperature");
```

## key_expr::format::keformat

Macro for formatting key expressions.

## key_expr::format::kewrite

Macro for writing formatted key expressions.
