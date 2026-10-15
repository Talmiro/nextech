---
name: graph-flow
description: A high-performance, type-safe framework for building multi-agent workflow systems in Rust. Use when building stateful AI agent orchestration, LLM-powered workflows, or interactive conversational applications. Based on graph execution patterns similar to LangGraph but built in Rust.
---

# graph-flow

A skill for building complex, stateful AI agent workflows in Rust using the graph-flow framework. This framework combines LangGraph's workflow design patterns with Rust's performance and type safety.

## When to Use This Skill

Use this skill when:
- Building AI agent orchestration systems
- Creating LLM-powered conversational workflows
- Implementing multi-step business processes with AI
- Building recommendation systems with RAG
- Creating insurance claims processing workflows
- Any stateful workflow that needs AI/LLM integration

## Quick Start

### 1. Add Dependency

```toml
# Cargo.toml
[dependencies]
graph-flow = { version = "0.2.3", features = ["rig"] }
```

### 2. Define a Task

```rust
use async_trait::async_trait;
use graph_flow::{Context, Task, TaskResult, NextAction};

struct HelloTask;

#[async_trait]
impl Task for HelloTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let name: String = context.get_sync("name").unwrap();
        let greeting = format!("Hello, {}", name);
        
        context.set("greeting", greeting.clone()).await;
        
        Ok(TaskResult::new(Some(greeting), NextAction::Continue))
    }
}
```

### 3. Build the Graph

```rust
use graph_flow::{GraphBuilder, InMemorySessionStorage, FlowRunner};
use std::sync::Arc;

let hello_task = Arc::new(HelloTask);

let graph = Arc::new(GraphBuilder::new("greeting_workflow")
    .add_task(hello_task.clone())
    .build());
```

### 4. Execute the Workflow

```rust
let session_storage = Arc::new(InMemorySessionStorage::new());
let flow_runner = FlowRunner::new(graph.clone(), session_storage.clone());

let session = Session::new_from_task("session_001".to_string(), hello_task.id());
session.context.set("name", "Batman".to_string()).await;
session_storage.save(session).await?;

let result = flow_runner.run("session_001").await?;
```

## Core Concepts

### Tasks
- Building blocks of your workflow
- Implement the `Task` trait with `run(context: Context) -> TaskResult`
- Each task can read/write to shared context
- Control flow via `NextAction` enum

### Context
- Thread-safe state management between tasks
- Store and retrieve typed data
- Built-in chat history management
- LLM integration via Rig

### Graph
- Defines workflow structure
- Connects tasks via edges
- Supports conditional routing
- Manages execution flow

### Session
- Maintains workflow state across interactions
- Persists between user interactions
- Stores context, current task, and status

## Key Features

1. **Type-safe**: Full Rust type safety for workflow data
2. **Stateful**: Sessions persist across interactions
3. **LLM Integration**: Seamless Rig integration for AI capabilities
4. **Conditional Routing**: Dynamic branching based on runtime data
5. **Human-in-the-loop**: Pause and wait for user input
6. **Storage Backends**: In-memory (dev) or PostgreSQL (prod)

## References

### Getting Started
- [Architecture Overview](./references/architecture.md) - High-level architecture and components
- [Task System](./references/task-system.md) - Creating and implementing tasks
- [Graph Construction](./references/graph-construction.md) - Building workflow graphs

### Advanced Topics
- [Session Management](./references/session-management.md) - Managing workflow state
- [LLM Integration](./references/llm-integration.md) - Integrating with LLMs using Rig
- [Storage Backends](./references/storage.md) - In-memory and PostgreSQL storage
- [FanOut Parallel Execution](./references/fanout.md) - Running tasks in parallel

### Examples
- [Examples Guide](./references/examples.md) - Detailed examples from basic to advanced
- [Insurance Claims Service](./references/insurance-example.md) - Complete real-world example
- [Recommendation Service](./references/recommendation-example.md) - RAG-based example

## Commands

### Build and Run
```bash
# Build all workspace members
cargo build

# Run specific services
cargo run --bin insurance-claims-service
cargo run --bin recommendation-service

# Run examples
cargo run --bin simple_example
cargo run --bin complex_example
cargo run --bin recommendation_flow
cargo run --bin fanout_basic
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p graph-flow
```

### Environment Setup
Required environment variables:
- `OPENROUTER_API_KEY`: For LLM integration via OpenRouter
- `DATABASE_URL`: PostgreSQL connection string (optional)

## Next Steps

1. Start with [Architecture Overview](./references/architecture.md) to understand the framework
2. Follow [Task System](./references/task-system.md) to create your first task
3. Build your graph using [Graph Construction](./references/graph-construction.md)
4. Add LLM capabilities with [LLM Integration](./references/llm-integration.md)
5. Deploy to production with [Storage Backends](./references/storage.md)

## Related Skills

- `/deep-research` - Research capabilities for gathering information
- `/boost-intel` - Stress-test research findings
