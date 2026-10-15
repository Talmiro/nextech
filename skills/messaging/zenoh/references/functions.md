# Functions

## open

Open a zenoh Session.

```rust
pub fn open<TryIntoConfig>(config: TryIntoConfig) -> OpenBuilder<TryIntoConfig>
where
    TryIntoConfig: TryInto<Config> + Send + 'static,
    <TryIntoConfig as TryInto<Config>>::Error: Debug,
```

```rust
let session = zenoh::open(Config::default()).await.unwrap();
```

## scout

Start scouting for zenoh peers/routers.

```rust
pub fn scout(
    whatami: WhatAmIMatcher,
    config: Config,
) -> ScoutBuilder {
    /* ... */
}
```

```rust
use zenoh::config::WhatAmI;

let receiver = zenoh::scout(WhatAmI::Peer | WhatAmI::Router, Config::default())
    .with(flume::bounded(32))
    .await
    .unwrap();
```

## init_log_from_env_or

Initialize logging from environment or default.

```rust
pub fn init_log_from_env_or(default: tracing::Level) {
    /* ... */
}
```

## try_init_log_from_env

Try to initialize logging from environment.

```rust
pub fn try_init_log_from_env() -> Result<(), LogInitError> {
    /* ... */
}
```

## scouting::scout

Module-level scout function (same as top-level).

## shm::cleanup_orphaned_shm_segments

Clean up orphaned shared memory segments.

```rust
pub async fn cleanup_orphaned_shm_segments() -> Result<(), ZShmError> {
    /* ... */
}
```
