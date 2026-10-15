# Session Management

Sessions maintain workflow state across multiple interactions. They persist between user requests and track where the workflow is and what data has been collected.

## What is a Session?

A session contains:
- **Unique ID**: Identifies the session
- **Graph ID**: Which workflow this session belongs to
- **Current Task**: Where in the workflow we are
- **Context**: All collected data and chat history
- **Status Message**: Current state description

## Creating a Session

```rust
use graph_flow::Session;

// Create new session starting at a specific task
let session = Session::new_from_task(
    "session_001".to_string(),
    start_task.id()
);

// Set initial context data
session.context.set("user_id", 12345).await;
session.context.set("language", "en".to_string()).await;
```

## Session Lifecycle

### 1. Creation
```rust
let session = Session::new_from_task(
    session_id.clone(),
    first_task.id()
);

// Initialize with starting data
session.context.set("started_at", chrono::Utc::now().to_rfc3339()).await;
```

### 2. Execution Loop
```rust
let flow_runner = FlowRunner::new(graph.clone(), storage.clone());

loop {
    let result = flow_runner.run(&session_id).await?;
    
    match result.status {
        ExecutionStatus::Completed => {
            println!("Workflow done: {:?}", result.response);
            break;
        }
        ExecutionStatus::WaitingForInput => {
            // Get user input, update context, continue
            // For web services: return response to client
            break;
        }
        ExecutionStatus::Paused { next_task_id, reason } => {
            // Automatically continues to next_task_id
            continue;
        }
        ExecutionStatus::Error(e) => {
            eprintln!("Error: {}", e);
            break;
        }
    }
}
```

### 3. Persistence
```rust
// FlowRunner handles this automatically
let result = flow_runner.run(&session_id).await?;
// Session is saved after each execution
```

### 4. Resumption
```rust
// Load existing session
let session = storage.get(&session_id).await?
    .expect("Session not found");

// Continue execution
let result = graph.execute_session(&mut session).await?;
storage.save(session).await?;
```

## Manual Session Management

For more control, manage sessions directly:

```rust
use graph_flow::{Graph, SessionStorage, InMemorySessionStorage};

// Load
let mut session = storage.get("session_id").await?
    .ok_or("Session not found")?;

// Execute
let result = graph.execute_session(&mut session).await?;

// Save
storage.save(session).await?;

// Check status
match result.status {
    ExecutionStatus::Completed => { /* done */ }
    ExecutionStatus::Paused { next_task_id, reason } => {
        println!("Paused at {}. Will continue to {}", next_task_id, reason);
    }
    // ...
}
```

## Session Storage

### In-Memory Storage (Development)

```rust
use graph_flow::InMemorySessionStorage;

let storage: Arc<dyn SessionStorage> = Arc::new(InMemorySessionStorage::new());
```

**Pros:**
- Fast
- Simple
- No setup required

**Cons:**
- Data lost on restart
- Not suitable for production

### PostgreSQL Storage (Production)

```rust
use graph_flow::PostgresSessionStorage;

let storage: Arc<dyn SessionStorage> = Arc::new(
    PostgresSessionStorage::connect("postgresql://user:pass@localhost/db").await?
);
```

**Pros:**
- Persistent
- Scalable
- Production-ready

**Cons:**
- Requires database setup
- Slower than in-memory

### Switching Storage

Both implement the same trait:

```rust
use graph_flow::SessionStorage;

fn create_storage() -> Arc<dyn SessionStorage> {
    if std::env::var("DATABASE_URL").is_ok() {
        Arc::new(PostgresSessionStorage::connect(&db_url).await?)
    } else {
        Arc::new(InMemorySessionStorage::new())
    }
}
```

## Session Data

### Context Data

Store any serializable data:

```rust
// Primitives
context.set("count", 42).await;
context.set("name", "Alice".to_string()).await;
context.set("active", true).await;

// Complex types
#[derive(Serialize, Deserialize)]
struct Claim {
    id: String,
    amount: f64,
    status: String,
}

context.set("claim", Claim { 
    id: "CLM-001".to_string(),
    amount: 500.0,
    status: "pending".to_string()
}).await;
```

### Chat History

Built-in conversation tracking:

```rust
// Add messages
context.add_user_message("I want to file a claim".to_string()).await;
context.add_assistant_message("I'm sorry to hear that. What happened?".to_string()).await;

// Retrieve
let history = context.get_chat_history().await;
let messages: Vec<SerializableMessage> = context.get_all_messages().await;
let recent = context.get_last_messages(5).await;

// Get count
let count = context.chat_history_len().await;
```

### Limiting Chat History

Prevent unbounded growth:

```rust
// Create context with limit
let context = Context::with_max_chat_messages(100);  // Keep only last 100

// Messages are automatically pruned
for i in 0..200 {
    context.add_user_message(format!("Message {}", i)).await;
}

assert_eq!(context.chat_history_len().await, 100);  // Only last 100 kept
```

## Execution Patterns

### Interactive (Web Service)

```rust
async fn handle_request(
    State(state): State<AppState>,
    Json(req): Json<ExecuteRequest>,
) -> Result<Json<Response>, StatusCode> {
    // Get or create session
    let session_id = req.session_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    // If new session, create it
    if !storage.exists(&session_id).await? {
        let session = Session::new_from_task(session_id.clone(), start_task.id());
        storage.save(session).await?;
    }
    
    // Set user input
    if let Some(input) = req.content {
        let mut session = storage.get(&session_id).await?.unwrap();
        session.context.set("user_input", input).await;
        storage.save(session).await?;
    }
    
    // Execute
    let result = state.flow_runner.run(&session_id).await?;
    
    Ok(Json(Response {
        session_id,
        response: result.response,
        status: format!("{:?}", result.status),
    }))
}
```

### Batch Processing

```rust
async fn process_batch(
    items: Vec<BatchItem>,
    graph: Arc<Graph>,
    storage: Arc<dyn SessionStorage>,
) -> Vec<BatchResult> {
    let mut results = Vec::new();
    
    for item in items {
        // Create session for each item
        let session_id = format!("batch_{}", item.id);
        let session = Session::new_from_task(session_id.clone(), start_task.id());
        session.context.set("item_data", item.data).await;
        storage.save(session).await?;
        
        // Run to completion
        let flow_runner = FlowRunner::new(graph.clone(), storage.clone());
        
        loop {
            let result = flow_runner.run(&session_id).await?;
            
            match result.status {
                ExecutionStatus::Completed => {
                    results.push(BatchResult {
                        id: item.id,
                        response: result.response,
                        success: true,
                    });
                    break;
                }
                ExecutionStatus::Error(e) => {
                    results.push(BatchResult {
                        id: item.id,
                        response: Some(e),
                        success: false,
                    });
                    break;
                }
                _ => continue,
            }
        }
    }
    
    results
}
```

## Session Best Practices

1. **Use FlowRunner**: For most web services, use FlowRunner to handle loading/saving automatically

2. **Create Session Once**: Create the session on first user interaction, then reference by ID

3. **Store Meaningful Data**: Put all workflow-relevant data in context, not in external storage

4. **Use Typed Keys**: Define constants for context keys to avoid typos:

```rust
mod session_keys {
    pub const USER_INPUT: &str = "user_input";
    pub const INSURANCE_TYPE: &str = "insurance_type";
    pub const CLAIM_DETAILS: &str = "claim_details";
}

// Usage
context.get::<String>(session_keys::USER_INPUT).await;
```

5. **Handle Session Not Found**: Always handle the case where a session doesn't exist

## Next Steps

- [LLM Integration](./llm-integration.md) - AI capabilities in workflows
- [Storage Backends](./storage.md) - Deep dive into storage options
- [Examples](./examples.md) - Complete examples
