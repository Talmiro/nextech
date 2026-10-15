# Architecture Overview

This document provides a high-level overview of the graph-flow framework architecture.

## What is graph-flow?

graph-flow is a Rust framework for building stateful, multi-step AI agent workflows. It combines:
- **Graph execution** - for orchestrating complex workflows
- **LLM integration** - via Rig crate for AI capabilities
- **Type safety** - Rust's compile-time guarantees
- **Stateful sessions** - persist across interactions

## Core Components

### 1. Task (`Task` trait)

Tasks are the fundamental building blocks of any workflow. Each task:
- Implements the `Task` trait with `async fn run(context: Context)`
- Returns a `TaskResult` containing response and `NextAction`
- Can read/write to the shared context

```rust
#[async_trait]
impl Task for MyTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Your logic here
        Ok(TaskResult::new(Some(response), NextAction::Continue))
    }
}
```

### 2. Context

The context provides thread-safe state management:
- **Data Storage**: `context.set(key, value)` and `context.get<T>(key)`
- **Chat History**: Built-in conversation tracking
- **LLM Integration**: Convert to Rig messages for AI calls

```rust
// Store data
context.set("user_name", "Alice".to_string()).await;

// Retrieve data
let name: Option<String> = context.get("user_name").await;

// Chat history
context.add_user_message("Hello".to_string()).await;
context.add_assistant_message("Hi there!".to_string()).await;
```

### 3. Graph

The graph defines workflow structure:
- Contains all tasks and their connections
- Supports linear and conditional edges
- Manages execution flow

```rust
let graph = GraphBuilder::new("my_workflow")
    .add_task(task1)
    .add_task(task2)
    .add_edge(task1.id(), task2.id())
    .build();
```

### 4. Session

Sessions maintain workflow state:
- Unique ID for tracking
- Current task position
- Context data
- Status messages

```rust
let session = Session::new_from_task(
    "session_123".to_string(),
    start_task.id()
);
session.context.set("initial_data", value).await;
```

### 5. Storage

Storage backends persist sessions:
- **InMemorySessionStorage**: Fast, non-persistent (development)
- **PostgresSessionStorage**: Persistent, scalable (production)

### 6. FlowRunner

High-level API for execution:
- Handles load → execute → save pattern
- Ideal for web services
- Reduces boilerplate

```rust
let runner = FlowRunner::new(graph, storage);
let result = runner.run("session_id").await?;
```

## Execution Flow

```
User Request
    │
    ▼
┌─────────────────────┐
│   Load Session      │ ◄── From storage (or create new)
└─────────────────────┘
    │
    ▼
┌─────────────────────┐
│  Execute Current    │ ◄── Run Task::run() with context
│       Task          │
└─────────────────────┘
    │
    ▼
┌─────────────────────┐
│  Determine Next     │ ◄── Based on NextAction
│      Action         │
└─────────────────────┘
    │
    ├── Continue ──────► Find next task, pause (return to caller)
    │
    ├── ContinueAndExecute ──► Execute next task immediately
    │
    ├── WaitForInput ────► Pause, wait for user input
    │
    ├── GoTo(task_id) ► Jump to specific task
    │
    └── End ───────────► Workflow complete
    │
    ▼
┌─────────────────────┐
│   Save Session      │ ◄── Persist to storage
└─────────────────────┘
    │
    ▼
  Response to User
```

## Directory Structure

```
rs-graph-llm/
├── graph-flow/                    # Core framework
│   ├── src/
│   │   ├── task.rs              # Task trait and NextAction
│   │   ├── context.rs           # Context for state management
│   │   ├── graph.rs             # Graph and GraphBuilder
│   │   ├── runner.rs            # FlowRunner
│   │   ├── storage.rs           # In-memory storage
│   │   ├── storage_postgres.rs  # PostgreSQL storage
│   │   └── ...
│   └── Cargo.toml
│
├── examples/                     # Learning examples
│   ├── simple_example.rs
│   ├── complex_example.rs
│   ├── recommendation_flow.rs
│   └── fanout_basic.rs
│
├── insurance-claims-service/      # Real-world example
│   └── src/tasks/
│
└── recommendation-service/       # RAG example
    └── src/tasks/
```

## Key Design Decisions

### 1. Stateless Tasks
Tasks should be stateless - all state goes in Context. This allows:
- Easy testing
- Parallel execution potential
- Clean separation of concerns

### 2. Async Trait
The `Task` trait uses `async_trait` for:
- Async/await support
- Dynamic dispatch
- Trait objects support

### 3. Arc<dyn Task>
Tasks are wrapped in `Arc` for:
- Shared ownership
- Cloning without execution
- Thread safety

### 4. NextAction Enum
Controls flow explicitly:
- **Continue**: Step-by-step (interactive)
- **ContinueAndExecute**: Continuous (batch)
- **WaitForInput**: Human-in-the-loop
- **GoTo**: Dynamic routing
- **End**: Complete workflow

## Integration Points

### LLM Integration (via Rig)
```rust
let agent = client.agent("openai/gpt-4o-mini")
    .preamble("You are a helpful assistant")
    .build();

let messages = context.get_rig_messages().await;
let response = agent.chat(&user_input, messages).await?;
```

### HTTP Service (via Axum)
```rust
async fn execute(
    State(state): State<AppState>,
    Json(request): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, StatusCode> {
    let result = state.flow_runner.run(&request.session_id).await?;
    Ok(Json(ExecuteResponse { ... }))
}
```

### Database (PostgreSQL)
```rust
let storage = PostgresSessionStorage::connect(&database_url).await?;
```

## Next Steps

- [Task System](./task-system.md) - Learn to create tasks
- [Graph Construction](./graph-construction.md) - Build workflows
- [Session Management](./session-management.md) - Handle state
- [LLM Integration](./llm-integration.md) - Add AI capabilities
