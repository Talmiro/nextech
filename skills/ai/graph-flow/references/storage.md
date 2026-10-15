# Storage Backends

graph-flow supports multiple storage backends for persisting workflow sessions. This document covers the available options and how to use them.

## SessionStorage Trait

All storage backends implement the `SessionStorage` trait:

```rust
pub trait SessionStorage: Send + Sync {
    async fn get(&self, id: &str) -> Result<Option<Session>>;
    async fn save(&self, session: Session) -> Result<()>;
    async fn delete(&self, id: &str) -> Result<()>;
    async fn exists(&self, id: &str) -> Result<bool>;
}
```

## In-Memory Storage

Best for development and testing.

### Setup

```rust
use graph_flow::InMemorySessionStorage;

let storage: Arc<dyn SessionStorage> = Arc::new(InMemorySessionStorage::new());
```

### Usage

```rust
use graph_flow::{Session, Task};

let session = Session::new_from_task(
    "session_001".to_string(),
    start_task.id()
);

// Save
storage.save(session).await?;

// Load
let loaded = storage.get("session_001").await?;
assert!(loaded.is_some());

// Check exists
let exists = storage.exists("session_001").await?;

// Delete
storage.delete("session_001").await?;
```

### Full Example

```rust
use graph_flow::{GraphBuilder, FlowRunner, Session, InMemorySessionStorage, Task, TaskResult, NextAction};
use std::sync::Arc;
use async_trait::async_trait;

struct HelloTask;

#[async_trait]
impl Task for HelloTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        context.set("greeting", "Hello, World!".to_string()).await;
        Ok(TaskResult::new(None, NextAction::End))
    }
}

#[tokio::main]
async fn main() -> graph_flow::Result<()> {
    let storage: Arc<dyn SessionStorage> = Arc::new(InMemorySessionStorage::new());
    let task = Arc::new(HelloTask);
    
    let graph = Arc::new(GraphBuilder::new("test")
        .add_task(task.clone())
        .build());
    
    // Create session
    let session = Session::new_from_task("test_session".to_string(), task.id());
    storage.save(session).await?;
    
    // Run workflow
    let runner = FlowRunner::new(graph, storage);
    let result = runner.run("test_session").await?;
    
    println!("Result: {:?}", result.response);
    
    Ok(())
}
```

## PostgreSQL Storage

Best for production environments.

### Setup

```toml
# Cargo.toml
graph-flow = { version = "0.2.3", features = ["postgres"] }

# Also add sqlx directly if needed
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres"] }
```

### Connection

```rust
use graph_flow::PostgresSessionStorage;

let storage: Arc<dyn SessionStorage> = Arc::new(
    PostgresSessionStorage::connect("postgresql://user:password@localhost/mydb").await?
);
```

### Environment Variable

```bash
export DATABASE_URL="postgresql://user:password@localhost/mydb"
```

### With Pool Configuration

```rust
use sqlx::{postgres::PgPoolOptions, PgPool};

let pool = PgPoolOptions::new()
    .max_connections(10)
    .connect("postgresql://user:password@localhost/mydb")
    .await?;

let storage = Arc::new(
    PostgresSessionStorage::from_pool(pool)
);
```

### Schema

PostgreSQL storage automatically creates the required table:

```sql
CREATE TABLE IF NOT EXISTS graph_flow_sessions (
    id VARCHAR(255) PRIMARY KEY,
    graph_id VARCHAR(255) NOT NULL,
    current_task_id VARCHAR(255) NOT NULL,
    context JSONB NOT NULL DEFAULT '{}',
    status_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_sessions_updated ON graph_flow_sessions(updated_at DESC);
```

## Switching Backends

Easily switch between storage backends:

```rust
fn create_storage() -> Arc<dyn SessionStorage> {
    match std::env::var("DATABASE_URL") {
        Ok(url) => {
            // Use PostgreSQL in production
            Arc::new(
                tokio::runtime::Runtime::new()
                    .unwrap()
                    .block_on(PostgresSessionStorage::connect(&url))
                    .expect("Failed to connect to PostgreSQL")
            )
        }
        Err(_) => {
            // Use in-memory for development
            Arc::new(InMemorySessionStorage::new())
        }
    }
}
```

## HTTP Service Example

Complete example with Axum:

```rust
use axum::{
    Router, State, Json,
    routing::post,
    extract::Path,
};
use graph_flow::{FlowRunner, InMemorySessionStorage, Session, SessionStorage};
use std::sync::Arc;

struct AppState {
    storage: Arc<dyn SessionStorage>,
    flow_runner: FlowRunner,
}

#[derive(Deserialize)]
struct ExecuteRequest {
    session_id: Option<String>,
    content: Option<String>,
}

#[derive(Serialize)]
struct ExecuteResponse {
    session_id: String,
    response: Option<String>,
    status: String,
}

async fn execute(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, String> {
    // Get or create session
    let session_id = match req.session_id {
        Some(id) => id,
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            let session = Session::new_from_task(id.clone(), "start_task");
            state.storage.save(session).await
                .map_err(|e| e.to_string())?;
            id
        }
    };
    
    // Set user input if provided
    if let Some(content) = req.content {
        if let Some(mut session) = state.storage.get(&session_id).await
            .map_err(|e| e.to_string())?
        {
            session.context.set("user_input", content).await;
            state.storage.save(session).await
                .map_err(|e| e.to_string())?;
        }
    }
    
    // Execute workflow step
    let result = state.flow_runner.run(&session_id).await
        .map_err(|e| e.to_string())?;
    
    Ok(Json(ExecuteResponse {
        session_id,
        response: result.response,
        status: format!("{:?}", result.status),
    }))
}

async fn get_session(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
) -> Result<Json<serde_json::Value>, String> {
    let session = state.storage.get(&session_id).await
        .map_err(|e| e.to_string())?
        .ok_or("Session not found")?;
    
    let json = serde_json::json!({
        "id": session.id,
        "graph_id": session.graph_id,
        "current_task_id": session.current_task_id,
        "context": session.context,
        "status_message": session.status_message,
    });
    
    Ok(Json(json))
}

#[tokio::main]
async fn main() {
    let storage: Arc<dyn SessionStorage> = Arc::new(InMemorySessionStorage::new());
    // Note: You'd create the graph here
    let flow_runner = FlowRunner::new(Arc::new(Graph::new("test")), storage.clone());
    
    let state = Arc::new(AppState {
        storage,
        flow_runner,
    });
    
    let app = Router::new()
        .route("/execute", post(execute))
        .route("/session/:id", get_session)
        .with_state(state);
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Best Practices

### 1. Choose Right Backend

| Use Case | Backend |
|----------|---------|
| Development | InMemory |
| Testing | InMemory |
| Production | PostgreSQL |
| Serverless | PostgreSQL |

### 2. Handle Connection Errors

```rust
async fn get_storage() -> Result<Arc<dyn SessionStorage>, Box<dyn Error>> {
    let storage = if let Ok(url) = std::env::var("DATABASE_URL") {
        Arc::new(
            PostgresSessionStorage::connect(&url).await?
        ) as Arc<dyn SessionStorage>
    } else {
        Arc::new(InMemorySessionStorage::new())
    };
    
    Ok(storage)
}
```

### 3. Session Cleanup

For long-running services, clean up old sessions:

```rust
async fn cleanup_old_sessions(storage: &Arc<dyn SessionStorage>) {
    // This would require adding a cleanup method to the trait
    // Or use raw SQL: DELETE FROM graph_flow_sessions 
    // WHERE updated_at < NOW() - INTERVAL '7 days';
}
```

### 4. Error Handling

```rust
match storage.get(&session_id).await {
    Ok(Some(session)) => { /* use session */ }
    Ok(None) => return Err(Error::SessionNotFound(session_id)),
    Err(e) => return Err(Error::StorageError(e.to_string())),
}
```

## Next Steps

- [Examples](./examples.md) - Complete examples with storage
- [Session Management](./session-management.md) - Working with sessions
- [LLM Integration](./llm-integration.md) - AI capabilities
