# Liveliness and Matching Modules

## Liveliness Module

The Liveliness API allows applications to declare tokens whose liveliness is tied to the Zenoh Session. Remote applications can query and subscribe to liveliness changes.

### Liveliness

Main entry point for liveliness operations on a Session.

```rust
let session = zenoh::open(zenoh::Config::default()).await.unwrap();
let liveliness = session.liveliness();
```

**Key Methods:**
- `declare_token(key_expr)` - Create a LivelinessToken
- `declare_subscriber(key_expr)` - Subscribe to liveliness changes
- `get(key_expr)` - Query liveliness tokens

---

### LivelinessToken

A token whose liveliness is tied to the Zenoh Session. Automatically undeclared when dropped.

```rust
let token = session
    .liveliness()
    .declare_token("key/expression")
    .await
    .unwrap();
```

**Methods:**
- `undeclare()` - Undeclare the token

---

### LivelinessTokenBuilder

Builder for initializing a LivelinessToken.

```rust
session.liveliness()
    .declare_token("key/expression")
    .await
    .unwrap();
```

---

### LivelinessGetBuilder

Builder for querying liveliness tokens.

**Methods:**
- `callback(callback)` - Set callback for replies
- `callback_mut(callback)` - Set mutable callback
- `with(handler)` - Set custom handler
- `timeout(duration)` - Set query timeout
- `cancellation_token(token)` - Set cancellation token

```rust
let replies = session
    .liveliness()
    .get("key/**")
    .await
    .unwrap();
```

---

### LivelinessSubscriberBuilder

Builder for subscribing to liveliness changes.

**Methods:**
- `callback(callback)` - Set callback for samples
- `callback_mut(callback)` - Set mutable callback
- `with(handler)` - Set custom handler
- `background()` - Run in background (no return value)
- `history(bool)` - Query currently live tokens on subscribe

```rust
let subscriber = session
    .liveliness()
    .declare_subscriber("key/**")
    .await
    .unwrap();
```

---

## Matching Module

The Matching API allows monitoring when Publishers/Queryables have matching Subscribers/Queryables.

### MatchingStatus

Indicates if there exist entities matching a key expression.

```rust
let status = publisher.matching_status().await.unwrap();
if status.matching() {
    println!("Has matching subscribers");
}
```

**Methods:**
- `matching()` - Returns `true` if matching entities exist

---

### MatchingListener

Listener that sends notifications when MatchingStatus changes.

```rust
let listener = publisher.matching_listener().await.unwrap();
while let Ok(status) = listener.recv_async().await {
    if status.matching() {
        println!("Subscribers available");
    } else {
        println!("No subscribers");
    }
}
```

**Methods:**
- `undeclare()` - Undeclare the listener
- `handler()` - Get handler reference
- `handler_mut()` - Get mutable handler reference

---

### MatchingListenerBuilder

Builder for initializing a MatchingListener.

**Methods:**
- `callback(callback)` - Set callback for matching status
- `callback_mut(callback)` - Set mutable callback
- `with(handler)` - Set custom handler
- `background()` - Run in background

```rust
publisher
    .matching_listener()
    .callback(|status| {
        println!("Matching: {}", status.matching());
    })
    .await
    .unwrap();
```
