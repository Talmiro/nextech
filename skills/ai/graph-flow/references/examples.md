# Examples Guide

This document provides detailed examples from basic to advanced, showing how to build various types of workflows.

## Simple Example

The most basic workflow with a single task:

```rust
use graph_flow::{Task, TaskResult, NextAction, Context, GraphBuilder, InMemorySessionStorage, FlowRunner, Session};
use async_trait::async_trait;
use std::sync::Arc;

struct HelloTask;

#[async_trait]
impl Task for HelloTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let name: String = context.get_sync("name").unwrap_or("World".to_string());
        let greeting = format!("Hello, {}!", name);
        
        context.set("greeting", greeting.clone()).await;
        
        Ok(TaskResult::new(Some(greeting), NextAction::End))
    }
}

#[tokio::main]
async fn main() -> graph_flow::Result<()> {
    // Create task
    let task = Arc::new(HelloTask);
    
    // Build graph
    let graph = Arc::new(GraphBuilder::new("hello_workflow")
        .add_task(task.clone())
        .build());
    
    // Create storage and runner
    let storage = Arc::new(InMemorySessionStorage::new());
    let runner = FlowRunner::new(graph, storage.clone());
    
    // Create session
    let session = Session::new_from_task("session_001".to_string(), task.id());
    session.context.set("name", "Alice".to_string()).await;
    storage.save(session).await?;
    
    // Execute
    let result = runner.run("session_001").await?;
    
    println!("Response: {:?}", result.response);
    // Output: Response: Some("Hello, Alice!")
    
    Ok(())
}
```

## Linear Workflow

Multiple tasks in sequence:

```rust
struct StepOneTask;

#[async_trait]
impl Task for StepOneTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        context.set("step", 1).await;
        Ok(TaskResult::new(Some("Step 1 complete".to_string()), NextAction::Continue))
    }
}

struct StepTwoTask;

#[async_trait]
impl Task for StepTwoTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let step: i32 = context.get_sync("step").unwrap_or(0);
        context.set("step", step + 1).await;
        
        Ok(TaskResult::new(Some("Step 2 complete".to_string()), NextAction::Continue))
    }
}

struct StepThreeTask;

#[async_trait]
impl Task for StepThreeTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let steps: i32 = context.get_sync("step").unwrap_or(0);
        
        Ok(TaskResult::new(
            Some(format!("All {} steps complete!", steps)),
            NextAction::End
        ))
    }
}

// Build linear workflow
let graph = GraphBuilder::new("linear_workflow")
    .add_task(Arc::new(StepOneTask))
    .add_task(Arc::new(StepTwoTask))
    .add_task(Arc::new(StepThreeTask))
    .add_edge("StepOneTask", "StepTwoTask")
    .add_edge("StepTwoTask", "StepThreeTask")
    .build();
```

## Conditional Workflow

Routing based on data:

```rust
struct ClassifyTask;

#[async_trait]
impl Task for ClassifyTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Simple classification logic
        let input: String = context.get_sync("input").unwrap_or_default();
        
        let category = if input.to_lowercase().contains("urgent") {
            "urgent"
        } else {
            "normal"
        };
        
        context.set("category", category.to_string()).await;
        
        Ok(TaskResult::new(
            Some(format!("Category: {}", category)),
            NextAction::Continue
        ))
    }
}

struct UrgentTask;

#[async_trait]
impl Task for UrgentTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        Ok(TaskResult::new(
            Some("Urgent path - immediate attention required!".to_string()),
            NextAction::End
        ))
    }
}

struct NormalTask;

#[async_trait]
impl Task for NormalTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        Ok(TaskResult::new(
            Some("Normal path - will be processed in order.".to_string()),
            NextAction::End
        ))
    }
}

// Build conditional workflow
let graph = GraphBuilder::new("conditional")
    .add_task(Arc::new(ClassifyTask))
    .add_task(Arc::new(UrgentTask))
    .add_task(Arc::new(NormalTask))
    .add_conditional_edge(
        "ClassifyTask",
        |ctx| ctx.get_sync::<String>("category")
            .map(|c| c == "urgent")
            .unwrap_or(false),
        "UrgentTask",
        "NormalTask"
    )
    .build();
```

## Human-in-the-Loop Workflow

Pausing for user input:

```rust
struct RequestApprovalTask;

#[async_trait]
impl Task for RequestApprovalTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let amount: f64 = context.get_sync("claim_amount").unwrap_or(0.0);
        
        if amount >= 1000.0 {
            // High value - need human approval
            Ok(TaskResult::new(
                Some(format!(
                    "Claim amount ${:.2} requires manager approval. Please review and approve/reject.",
                    amount
                )),
                NextAction::WaitForInput  // Pause here
            ))
        } else {
            // Low value - auto approve
            context.set("approved", true).await;
            Ok(TaskResult::new(
                Some(format!("Claim for ${:.2} auto-approved!", amount)),
                NextAction::Continue
            ))
        }
    }
}

struct ProcessApprovalTask;

#[async_trait]
impl Task for ProcessApprovalTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let decision: String = context.get_sync("approval_decision")
            .unwrap_or_default();
        
        let approved = decision.to_lowercase() == "approved";
        context.set("approved", approved).await;
        
        Ok(TaskResult::new(
            Some(format!("Claim {}!", if approved { "approved" } else { "rejected" })),
            NextAction::End
        ))
    }
}

// Web service handler would:
// 1. Call flow_runner.run() 
// 2. Get response with "WaitForInput" status
// 3. Return to user
// 4. Later receive approval decision
// 5. Update context with decision
// 6. Call flow_runner.run() again

async fn handle_approval(
    state: &AppState,
    session_id: &str,
    decision: String,
) -> Result<String, String> {
    // Load and update context
    let mut session = state.storage.get(session_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Session not found")?;
    
    session.context.set("approval_decision", decision).await;
    state.storage.save(session).await
        .map_err(|e| e.to_string())?;
    
    // Continue execution
    let result = state.flow_runner.run(session_id).await
        .map_err(|e| e.to_string())?;
    
    Ok(result.response.unwrap_or_default())
}
```

## LLM-Powered Workflow

Complete example with LLM integration:

```rust
use rig::{providers::openrouter, agent::Agent};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SentimentResult {
    sentiment: String,
    confidence: f64,
}

struct SentimentAnalysisTask;

#[async_trait]
impl Task for SentimentAnalysisTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let input: String = context.get_sync("user_input")
            .unwrap_or_default();
        
        // Call LLM
        let client = openrouter::Client::new(
            &std::env::var("OPENROUTER_API_KEY").unwrap()
        );
        let agent = client.agent("openai/gpt-4o-mini")
            .preamble("Analyze the sentiment. Return only 'positive', 'negative', or 'neutral'.")
            .build();
        
        let response = agent.completion(&input).await
            .map_err(|e| GraphError::TaskExecutionFailed(e.to_string()))?;
        
        let sentiment = response.trim().to_lowercase();
        let sentiment = if sentiment.contains("positive") {
            "positive"
        } else if sentiment.contains("negative") {
            "negative"
        } else {
            "neutral"
        };
        
        context.set("sentiment", sentiment.to_string()).await;
        
        Ok(TaskResult::new(
            Some(format!("Detected sentiment: {}", sentiment)),
            NextAction::Continue
        ))
    }
}

struct ResponseTask;

#[async_trait]
impl Task for ResponseTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let sentiment: String = context.get_sync("sentiment")
            .unwrap_or_default();
        
        let client = openrouter::Client::new(
            &std::env::var("OPENROUTER_API_KEY").unwrap()
        );
        let agent = client.agent("openai/gpt-4o-mini")
            .preamble("You are a helpful assistant.")
            .build();
        
        let prompt = format!(
            "The user expressed a {} sentiment. Write a brief, empathetic response.",
            sentiment
        );
        
        let response = agent.completion(&prompt).await
            .map_err(|e| GraphError::TaskExecutionFailed(e.to_string()))?;
        
        Ok(TaskResult::new(Some(response), NextAction::End))
    }
}

// Build workflow
let graph = GraphBuilder::new("sentiment_workflow")
    .add_task(Arc::new(SentimentAnalysisTask))
    .add_task(Arc::new(ResponseTask))
    .add_edge("SentimentAnalysisTask", "ResponseTask")
    .build();
```

## Error Handling Workflow

Handling errors gracefully:

```rust
struct ValidateInputTask;

#[async_trait]
impl Task for ValidateInputTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let input: String = context.get_sync("user_input")
            .unwrap_or_default();
        
        if input.is_empty() {
            return Ok(TaskResult::new(
                Some("Please provide input".to_string()),
                NextAction::WaitForInput
            ));
        }
        
        if input.len() < 3 {
            return Err(GraphError::TaskExecutionFailed(
                "Input too short (minimum 3 characters)".to_string()
            ));
        }
        
        context.set("validated_input", input).await;
        
        Ok(TaskResult::new(Some("Input validated".to_string()), NextAction::Continue))
    }
}

struct ProcessTask;

#[async_trait]
impl Task for ProcessTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let input: String = context.get_sync("validated_input")
            .unwrap_or_default();
        
        // Process...
        
        Ok(TaskResult::new(Some("Processed successfully".to_string()), NextAction::End))
    }
}

// Error handling in runner
let result = runner.run(&session_id).await;

match result {
    Ok(execution_result) => {
        match execution_result.status {
            ExecutionStatus::Completed => { /* done */ }
            ExecutionStatus::Error(e) => {
                eprintln!("Workflow error: {}", e);
                // Handle error - maybe start error recovery workflow
            }
            _ => { /* continue */ }
        }
    }
    Err(e) => {
        eprintln!("Execution failed: {}", e);
    }
}
```

## Parallel Execution (FanOut)

Running tasks concurrently:

```rust
use graph_flow::FanOutTask;

struct FetchDataTask;

#[async_trait]
impl Task for FetchDataTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        context.set("data_source", "fetched".to_string()).await;
        Ok(TaskResult::new(Some("Data fetched".to_string()), NextAction::End))
    }
}

struct ProcessDataTask;

#[async_trait]
impl Task for ProcessDataTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        context.set("data_processed", true).await;
        Ok(TaskResult::new(Some("Data processed".to_string()), NextAction::End))
    }
}

// Create parallel execution task
let parallel_tasks = FanOutTask::new("parallel", vec![
    Arc::new(FetchDataTask),
    Arc::new(ProcessDataTask),
]);

// Results available in context:
// - parallel.FetchDataTask.response
// - parallel.ProcessDataTask.response
```

## More Examples

See the actual examples in the repository:

- `examples/simple_example.rs` - Basic workflow
- `examples/complex_example.rs` - Conditional routing
- `examples/recommendation_flow.rs` - RAG workflow
- `examples/fanout_basic.rs` - Parallel execution
- `insurance-claims-service/` - Full production example
- `recommendation-service/` - RAG example

## Next Steps

- [Insurance Claims Example](./insurance-example.md) - Complete real-world workflow
- [Recommendation Service Example](./recommendation-example.md) - RAG-based system
- [Storage Backends](./storage.md) - Deploy to production
