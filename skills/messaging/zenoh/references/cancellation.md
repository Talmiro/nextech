# CancellationToken

The `CancellationToken` is a synchronization primitive used to interrupt get queries in Zenoh. This is an **unstable** feature that requires the `unstable` feature flag.

## Enabling

```toml
# Cargo.toml
[dependencies]
zenoh = { version = "1.0", features = ["unstable"] }
```

## Basic Usage

```rust
use zenoh::prelude::*;
use zenoh::cancellation::CancellationToken;

// Create a default token
let token = CancellationToken::default();

// Use with get query
let session = zenoh::open(Config::default()).await.unwrap();

let replies = session.get("demo/key")
    .callback(move |reply| {
        println!("Received: {:?}", reply);
    })
    .cancellation_token(token.clone())
    .await
    .unwrap();

// Later, cancel the query
token.cancel().await.unwrap();
```

## Checking Cancellation Status

```rust
let token = CancellationToken::default();

// Check if cancelled
if token.is_cancelled() {
    println!("Query was cancelled");
}

// Cancel the token
token.cancel().await.unwrap();

// Now it will show as cancelled
assert!(token.is_cancelled());
```

## With Timeout

```rust
use std::time::Duration;
use tokio::time::timeout;

let token = CancellationToken::default();
let session = zenoh::open(Config::default()).await.unwrap();

// Start a query with cancellation token
let get_future = session.get("demo/key")
    .callback(|reply| { /* handle */ })
    .cancellation_token(token.clone())
    .await
    .unwrap();

// Try to cancel within a timeout
let result = timeout(Duration::from_secs(5), token.cancel()).await;

match result {
    Ok(Ok(())) => println!("Cancelled successfully"),
    Ok(Err(e)) => println!("Cancel error: {:?}", e),
    Err(_) => println!("Cancel timed out"),
}
```

## Multiple Queries

A single cancellation token can be shared across multiple queries:

```rust
let token = CancellationToken::default();
let session = zenoh::open(Config::default()).await.unwrap();

// Start multiple queries with same token
session.get("demo/key1")
    .callback(|_| {})
    .cancellation_token(token.clone())
    .await
    .unwrap();

session.get("demo/key2")
    .callback(|_| {})
    .cancellation_token(token.clone())
    .await
    .unwrap();

// Cancel all at once
token.cancel().await.unwrap();
```

## With Callback Mut

Using `callback_mut` for mutable callback:

```rust
let token = CancellationToken::default();
let mut counter = 0;

session.get("demo/key")
    .callback_mut(move |reply| {
        counter += 1;
        println!("Reply {}: {:?}", counter, reply);
    })
    .cancellation_token(token)
    .await
    .unwrap();
```

## Use Cases

### Long-running Queries

```rust
// Cancel a long-running query if it takes too long
let token = CancellationToken::default();

let query = session.get("slow/data source")
    .timeout(Duration::from_secs(30))
    .callback(|r| { /* handle */ })
    .cancellation_token(token.clone())
    .await
    .unwrap();

// If user clicks "cancel", interrupt the query
user_cancel_button.on_click(move || {
    let token = token.clone();
    async move {
        token.cancel().await.unwrap();
    }
});
```

### User-initiated Cancellation

```rust
let (cancel_tx, mut cancel_rx) = tokio::sync::mpsc::channel::<()>(1);
let token = CancellationToken::default();

// Spawn the query task
let query_handle = tokio::spawn(async move {
    session.get("demo/key")
        .callback(|r| { /*...*/ })
        .cancellation_token(token.clone())
        .await
        .unwrap();
});

// Spawn cancellation listener
tokio::spawn(async move {
    while let Some(_) = cancel_rx.recv().await {
        token.cancel().await.unwrap();
    }
});
```

## Important Notes

- **Unstable API**: This feature requires `unstable` feature flag
- **Cloning**: `CancellationToken` can be cheaply cloned
- **Multiple cancellations**: Calling `cancel()` multiple times is safe
- **Blocking**: The `cancel()` call blocks until any executing callback finishes

## See Also

- [Session](session.md) - Get operations
- [Query/Reply](reqrep.md) - Query patterns
