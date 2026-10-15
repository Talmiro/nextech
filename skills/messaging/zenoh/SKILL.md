---
name: zenoh
description: >
  Zenoh is a Zero Overhead Pub/Sub, Query and Compute protocol. Use when building 
  distributed pub/sub systems, request/reply patterns, or geo-distributed edge computing.

  ## Quick Start
  ```rust
  use zenoh::config::Config;
  use zenoh::prelude::*;

  // Open a session
  let session = zenoh::open(Config::default()).await.unwrap();

  // Publish data
  let publisher = session.declare_publisher("demo/key").await.unwrap();
  publisher.put("Hello Zenoh!").await.unwrap();

  // Subscribe to data
  let subscriber = session.declare_subscriber("demo/key").await.unwrap();
  let sample = subscriber.recv_async().await.unwrap();
  ```

  ## Core Patterns (see references for full details)
  - **Pub/Sub**: Publisher/Subscriber with key expressions, filtering, QoS
  - **Query/Reply**: Querier/Queryable for RPC-style communication
  - **Key Expressions**: Hierarchical topic matching with wildcards
  - **Scouting**: Auto-discovery of peers and routers

  ## Core API Types (see references for full details)
  - **references/config.md** — Config struct for session configuration
  - **references/session.md** — Session: main Zenoh component for pub/sub and queries
  - **references/encoding.md** — Encoding: payload format descriptions
  - **references/zbytes.md** — ZBytes: data payload type (with reader/writer)
  - **references/cancellation.md** — CancellationToken: interrupt queries (unstable)
  - **references/endpoint_locator.md** — EndPoint, Locator, Notifier: connection config
  - **references/whatami_zid.md** — WhatAmI, WhatAmIMatcher, ZenohId: node identity
  - **references/handlers.md** — Handlers: DefaultHandler, FifoChannel, Callback
  - **references/sample_reply.md** — Sample, Reply, ReplyError: data delivery types
  - **references/pubsub_types.md** — Publisher, Subscriber, PublisherBuilder, SubscriberBuilder
  - **references/query_types.md** — Querier, Queryable, Selector, KeyExpr
  - **references/liveliness_matching.md** — Liveliness module and MatchingStatus
  - **references/scouting_qos_time.md** — Scout, Hello, Priority, CongestionControl, Reliability, Timestamp, NTP64
  - **references/session_types.md** — OpenBuilder, CloseBuilder, SessionInfo, Link, Transport, EntityGlobalId
  - **references/sample_types.md** — Sample, SampleBuilder, SourceInfo, SampleKind, Locality
  - **references/pubsub_builders.md** — PublicationBuilder, PublisherBuilder, SubscriberBuilder
  - **references/query_builders.md** — QuerierBuilder, QueryableBuilder, ReplyBuilder
  - **references/traits.md** — Resolvable, Wait, Resolve, IntoHandler, Canonize, Undeclarable
  - **references/functions.md** — open, scout, init_log_from_env_or
  - **references/macros.md** — ke, kedefine, keformat, kewrite
  - **references/shm_types.md** — ShmProvider, ZShm, ZShmMut (shared memory)
  - **references/constants.md** — GIT_VERSION constant
  - **references/additional_types.md** — Remaining types (KeyExpr tree, handlers::fifo, etc.)
  - **references/authentication.md** — User-password, TLS, Access Control (ACL)
  - **references/type_aliases.md** — All type aliases (Error, Result, etc.)
  - **references/examples.md** — Running examples and common patterns
  - **references/advanced_usage.md** — Advanced features (batching, priority, consolidation)

## Reference Files (patterns)
  - **references/pubsub.md** — Pub/Sub pattern (publisher, subscriber, QoS)
  - **references/reqrep.md** — Query/Reply pattern (querier, queryable)
  - **references/keyexpr.md** — Key expressions and matching
  - **references/scouting.md** — Discovery and peer/router scouting

  ## Running Examples
  ```bash
  cargo run --example <example_name>
  # Examples: z_pub, z_sub, z_queryable, z_querier
  ```

  
argument-hint: "[what-to-build]"
allowed-tools:
  - Read
  - Glob
  - Grep
  - Edit
  - Write
  - Bash
---

(End of file - total 53 lines)
