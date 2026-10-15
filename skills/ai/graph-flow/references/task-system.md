# Task System

The task system is the foundation of graph-flow. Tasks are the building blocks that define what your workflow does.

## Task Trait

All tasks must implement the `Task` trait:

```rust
use async_trait::async_trait;
use graph_flow::{Context, Task, TaskResult, NextAction};

struct MyTask;

#[async_trait]
impl Task for MyTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Your task logic here
        Ok(TaskResult::new(Some("Done".to_string()), NextAction::Continue))
    }
}
```

## Task ID

Each task needs a unique identifier. There are two ways to set it:

### 1. Default ID (Recommended)
The default implementation uses the type name:

```rust
struct MyTask;

#[async_trait]
impl Task for MyTask {
    // id() will be "my_module::MyTask"
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        Ok(TaskResult::new(None, NextAction::End))
    }
}
```

### 2. Custom ID
Override the `id()` method for custom identifiers:

```rust
struct GreetingTask;

#[async_trait]
impl Task for GreetingTask {
    fn id(&self) -> &str {
        "greeting"  // Custom ID
    }

    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // ...
    }
}
```

## TaskResult

Every task returns a `TaskResult` containing:

```rust
pub struct TaskResult {
    pub response: Option<String>,      // Response to send to user
    pub next_action: NextAction,       // What to do next
    pub task_id: String,               // Auto-set by engine
    pub status_message: Option<String>, // Debug/logging info
}
```

### Creating TaskResult

```rust
// Basic
let result = TaskResult::new(
    Some("Hello, World!".to_string()),
    NextAction::Continue
);

// With status message
let result = TaskResult::new_with_status(
    Some("Data processed".to_string()),
    NextAction::Continue,
    Some("Processing completed with 95% confidence".to_string())
);

// Convenience methods
let result = TaskResult::move_to_next();        // Continue
let result = TaskResult::move_to_next_direct(); // ContinueAndExecute
```

## NextAction

The `NextAction` enum controls workflow execution:

| Variant | Behavior | Use Case |
|---------|----------|----------|
| `Continue` | Pause after this task, return control to caller | Interactive apps, web services |
| `ContinueAndExecute` | Immediately run next task | Batch processing, automated flows |
| `WaitForInput` | Pause and wait for user input | Human-in-the-loop workflows |
| `GoTo(task_id)` | Jump to specific task | Error handling, loops |
| `GoBack` | Return to previous task | Retry logic |
| `End` | Complete the workflow | Final task |

### Examples

```rust
// Interactive - step by step
Ok(TaskResult::new(Some("Step 1 done".to_string()), NextAction::Continue))

// Automated - run continuously
Ok(TaskResult::new(Some("Processing...".to_string()), NextAction::ContinueAndExecute))

// Wait for human input
Ok(TaskResult::new(
    Some("Please review and approve".to_string()),
    NextAction::WaitForInput
))

// Jump to error handler
Ok(TaskResult::new(
    Some("Error detected".to_string()),
    NextAction::GoTo("error_handler".to_string())
))

// Complete workflow
Ok(TaskResult::new(Some("All done!".to_string()), NextAction::End))
```

## Context Access

Tasks access shared state through the Context:

### Reading Data

```rust
async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
    // Async get - preferred
    let name: Option<String> = context.get("name").await;
    
    // Sync get - for use in edge conditions
    let name_sync: Option<String> = context.get_sync("name");
    
    // With default value
    let name = context.get_sync("name").unwrap_or("World".to_string());
    
    Ok(TaskResult::new(None, NextAction::Continue))
}
```

### Writing Data

```rust
async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
    // Async set - preferred
    context.set("result", "processed".to_string()).await;
    
    // Sync set
    context.set_sync("counter", 42);
    
    Ok(TaskResult::new(None, NextAction::Continue))
}
```

### Typed Data

```rust
// Store complex types
#[derive(Serialize, Deserialize)]
struct UserData {
    id: u32,
    name: String,
}

context.set("user", UserData { id: 1, name: "Alice".to_string() }).await;

// Retrieve
let user: Option<UserData> = context.get("user").await;
```

## Complete Task Examples

### Simple Task

```rust
struct HelloTask;

#[async_trait]
impl Task for HelloTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let name: String = context.get_sync("name").unwrap_or("World".to_string());
        let greeting = format!("Hello, {}!", name);
        
        context.set("greeting", greeting.clone()).await;
        
        Ok(TaskResult::new(Some(greeting), NextAction::Continue))
    }
}
```

### Conditional Task

```rust
struct ConditionalTask;

#[async_trait]
impl Task for ConditionalTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let user_input: Option<String> = context.get("user_input").await;
        
        match user_input {
            Some(input) if !input.is_empty() => {
                context.set("processed", input.to_uppercase()).await;
                Ok(TaskResult::new(
                    Some("Input processed".to_string()),
                    NextAction::ContinueAndExecute
                ))
            }
            _ => {
                Ok(TaskResult::new(
                    Some("Please provide input".to_string()),
                    NextAction::WaitForInput
                ))
            }
        }
    }
}
```

### Task with Error Handling

```rust
struct ValidationTask {
    max_retries: usize,
}

#[async_trait]
impl Task for ValidationTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let data: Option<String> = context.get("data").await;
        let retry_count: usize = context.get("retry_count").await.unwrap_or(0);
        
        match data {
            Some(data) if self.validate(&data) => {
                context.set("retry_count", 0).await;
                Ok(TaskResult::new(
                    Some("Validation passed".to_string()),
                    NextAction::Continue
                ))
            }
            Some(_) if retry_count < self.max_retries => {
                context.set("retry_count", retry_count + 1).await;
                Ok(TaskResult::new(
                    Some("Validation failed, retrying...".to_string()),
                    NextAction::GoTo("data_input".to_string())
                ))
            }
            _ => {
                Err(GraphError::TaskExecutionFailed(
                    "Validation failed after max retries".to_string()
                ))
            }
        }
    }
}

impl ValidationTask {
    fn validate(&self, data: &str) -> bool {
        !data.is_empty() && data.len() > 5
    }
}
```

### Task with LLM Integration

```rust
struct LlmChatTask;

#[async_trait]
impl Task for LlmChatTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let user_input: String = context.get_sync("user_input").unwrap_or_default();
        
        // Get chat history for context
        let chat_history = context.get_rig_messages().await;
        
        // Create LLM agent (requires rig feature)
        let client = openrouter::Client::new(&api_key);
        let agent = client.agent("openai/gpt-4o-mini")
            .preamble("You are a helpful assistant")
            .build();
        
        // Call LLM
        let response = agent.chat(&user_input, chat_history).await?;
        
        // Store conversation
        context.add_user_message(user_input).await;
        context.add_assistant_message(response.clone()).await;
        
        Ok(TaskResult::new(Some(response), NextAction::Continue))
    }
}
```

## Best Practices

1. **Stateless Tasks**: Don't store state in the task struct; use Context
2. **Error Handling**: Return `Err(GraphError::...)` for failures
3. **Status Messages**: Use for debugging and logging
4. **Type Safety**: Leverage Rust's type system for data
5. **Idiomatic Async**: Use async/await throughout

## Next Steps

- [Graph Construction](./graph-construction.md) - Connect tasks into workflows
- [Context System](./context.md) - Deep dive into state management
- [LLM Integration](./llm-integration.md) - AI-powered tasks
