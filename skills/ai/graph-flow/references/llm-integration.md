# LLM Integration

graph-flow integrates seamlessly with LLMs through the Rig crate. This enables AI-powered workflows with natural language understanding and generation.

## Setup

### Add Dependency

```toml
# Cargo.toml
[dependencies]
graph-flow = { version = "0.2.3", features = ["rig"] }
rig-core = "0.32.0"
```

### Configure Provider

```rust
use rig::providers::openrouter;

let client = openrouter::Client::new(&std::env::var("OPENROUTER_API_KEY").unwrap());
```

## Creating an LLM Agent

```rust
use rig::{agent::Agent, providers::openrouter};

let client = openrouter::Client::new(&api_key);

let agent = client.agent("openai/gpt-4o-mini")
    .preamble("You are a helpful insurance assistant.")
    .build();
```

### Agent Configuration

```rust
// With custom temperature
let agent = client.agent("openai/gpt-4o-mini")
    .preamble("You are a helpful assistant.")
    .temperature(0.7)  // 0.0 to 1.0
    .max_tokens(1000)   // Limit response length
    .build();

// Different providers
let openai_agent = client.agent("openai/gpt-4o");

let anthropic_agent = client.agent("anthropic/claude-3-opus");

let google_agent = client.assistant("google/gemini-pro");
```

## Using with Context

### Basic Chat

```rust
struct LlmChatTask;

#[async_trait]
impl Task for LlmChatTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let user_input: String = context.get_sync("user_input").unwrap_or_default();
        
        // Get chat history for context
        let chat_history = context.get_rig_messages().await;
        
        // Call LLM
        let agent = create_agent();
        let response = agent.chat(&user_input, chat_history).await?;
        
        // Store conversation
        context.add_user_message(user_input).await;
        context.add_assistant_message(response.clone()).await;
        
        Ok(TaskResult::new(Some(response), NextAction::Continue))
    }
}

fn create_agent() -> impl Agent {
    let client = openrouter::Client::new(&api_key);
    client.agent("openai/gpt-4o-mini")
        .preamble("You are a helpful assistant.")
        .build()
}
```

### Structured Output

Parse LLM responses into structured data:

```rust
#[derive(Serialize, Deserialize)]
struct ClaimClassification {
    insurance_type: String,  // "car" or "apartment"
    confidence: f64,
    reasoning: String,
}

async fn classify_claim(&self, description: &str) -> Result<ClaimClassification> {
    let prompt = format!(
        r#"Classify this insurance claim. Return JSON with:
- insurance_type: "car" or "apartment"
- confidence: 0.0 to 1.0
- reasoning: brief explanation

Claim: {}"#,
        description
    );
    
    let response = self.agent.completion(&prompt).await?;
    
    // Parse JSON response
    let classification: ClaimClassification = serde_json::from_str(&response)?;
    
    Ok(classification)
}
```

### Multi-Turn Conversation

Maintain context across turns:

```rust
struct InsuranceQueryTask;

#[async_trait]
impl Task for InsuranceQueryTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let user_input: String = context.get_sync("user_input").unwrap_or_default();
        
        // Get full chat history
        let history = context.get_rig_messages().await;
        
        // Add system context if needed
        let mut messages = vec![Message::system(
            "You are an insurance claims assistant. \
             Help users file claims by asking clarifying questions."
        )];
        messages.extend(history);
        
        let response = self.agent.completion(&user_input, messages).await?;
        
        // Update chat history
        context.add_user_message(user_input).await;
        context.add_assistant_message(response.clone()).await;
        
        Ok(TaskResult::new(Some(response), NextAction::WaitForInput))
    }
}
```

## Complete Example: Insurance Classifier

```rust
use rig::{agent::Assistant, providers::openrouter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimInfo {
    pub insurance_type: String,
    pub confidence: f64,
}

struct InsuranceClassifierTask {
    agent: impl Assistant,
}

impl InsuranceClassifierTask {
    fn new() -> Self {
        let client = openrouter::Client::new(
            &std::env::var("OPENROUTER_API_KEY").unwrap()
        );
        let agent = client.assistant("openai/gpt-4o-mini")
            .preamble(
                "You are an insurance claim classifier. \
                 Analyze claim descriptions and classify them as either \
                 'car' or 'apartment' insurance claims."
            )
            .build();
        
        Self { agent }
    }
    
    async fn classify(&self, description: &str) -> Result<ClaimInfo, String> {
        let prompt = format!(
            "Classify this insurance claim: '{}'\n\
             Return ONLY 'car' or 'apartment'.",
            description
        );
        
        let response = self.agent.completion(&prompt).await
            .map_err(|e| e.to_string())?;
        
        let insurance_type = response.trim().to_lowercase();
        let insurance_type = if insurance_type.contains("car") {
            "car"
        } else {
            "apartment"
        };
        
        Ok(ClaimInfo {
            insurance_type: insurance_type.to_string(),
            confidence: 0.9,
        })
    }
}

#[async_trait]
impl Task for InsuranceClassifierTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let description: String = context.get_sync("claim_description")
            .unwrap_or_default();
        
        let claim_info = self.classify(&description).await
            .map_err(|e| GraphError::TaskExecutionFailed(e))?;
        
        // Store classification result
        context.set("insurance_type", &claim_info.insurance_type).await;
        context.set("classification_confidence", claim_info.confidence).await;
        
        let response = format!(
            "I've classified this as a {} insurance claim (confidence: {:.0}%)",
            claim_info.insurance_type,
            claim_info.confidence * 100.0
        );
        
        Ok(TaskResult::new(Some(response), NextAction::Continue))
    }
}
```

## Extraction Example

Extract structured data from user input:

```rust
struct DetailsExtractionTask;

#[derive(Serialize, Deserialize)]
struct ClaimDetails {
    incident_date: String,
    location: String,
    description: String,
    estimated_cost: f64,
}

#[async_trait]
impl Task for DetailsExtractionTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let user_input: String = context.get_sync("user_input").unwrap_or_default();
        
        let prompt = format!(
            r#"Extract claim details from this user input.
Return JSON with these fields:
- incident_date: When it happened
- location: Where it happened
- description: What happened
- estimated_cost: Estimated loss amount (just the number)

User input: {}

JSON:"#,
            user_input
        );
        
        let response = self.agent.completion(&prompt).await?;
        
        // Parse the JSON response
        let details: ClaimDetails = serde_json::from_str(&response)
            .map_err(|e| GraphError::TaskExecutionFailed(
                format!("Failed to parse: {}", e)
            ))?;
        
        context.set("claim_details", details).await;
        
        Ok(TaskResult::new(
            Some("Details extracted successfully".to_string()),
            NextAction::Continue
        ))
    }
}
```

## Best Practices

### 1. Handle Errors Gracefully

```rust
async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
    let result = self.classify(&input).await;
    
    match result {
        Ok(classification) => {
            context.set("classification", classification).await;
            Ok(TaskResult::new(Some("Done".to_string()), NextAction::Continue))
        }
        Err(e) => {
            // Don't crash the workflow
            Ok(TaskResult::new(
                Some("Sorry, I couldn't process that. Please try again.".to_string()),
                NextAction::WaitForInput
            ))
        }
    }
}
```

### 2. Use System Prompts Effectively

```rust
agent.preamble(
    "You are an expert insurance claims processor. \
     - Always ask for clarification when needed \
     - Be empathetic but professional \
     - Extract accurate information \
     - Never make up information"
).build();
```

### 3. Manage Context Length

```rust
// Only use recent messages for long conversations
let recent_messages = context.get_last_rig_messages(10).await;
let response = agent.chat(&input, recent_messages).await;
```

### 4. Add Retry Logic

```rust
async fn call_with_retry(&self, prompt: &str, max_retries: usize) -> Result<String> {
    for attempt in 0..max_retries {
        match self.agent.completion(prompt).await {
            Ok(response) => return Ok(response),
            Err(e) if attempt < max_retries - 1 => {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

## Next Steps

- [Examples](./examples.md) - Complete LLM-powered workflows
- [Storage Backends](./storage.md) - Persisting sessions
- [Task System](./task-system.md) - Creating more complex tasks
