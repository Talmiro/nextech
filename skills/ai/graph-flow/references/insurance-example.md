# Insurance Claims Service Example

This is a complete real-world example of building an insurance claims processing workflow with graph-flow.

## Overview

The insurance claims service demonstrates:
- LLM-driven natural language interactions
- Conditional routing based on claim type
- Business rule validation
- Human-in-the-loop approval for high-value claims
- State management throughout the workflow

## Workflow Structure

```
InitialClaimQuery → InsuranceTypeClassifier
                           │
              ┌────────────┴────────────┐
              │                         │
           (car)                    (apartment)
              │                         │
       CarInsuranceDetails    ApartmentInsuranceDetails
              └────────────┬────────────┘
                           │
                    SmartClaimValidator
                           │
              ┌────────────┴────────────┐
              │                         │
        (< $1000)                  (≥ $1000)
              │                         │
       Auto-Approved             Manual Review
              │                         │
              └────────────┬────────────┘
                           │
                    FinalSummary
```

## Task Definitions

### 1. Initial Claim Query

```rust
// insurance-claims-service/src/tasks/initial_claim_query.rs

use rig::{providers::openrouter, agent::Agent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialClaimInfo {
    pub claimant_name: Option<String>,
    pub brief_description: String,
    pub contact_preference: Option<String>,
}

struct InitialClaimQueryTask {
    agent: impl Agent,
}

impl InitialClaimQueryTask {
    fn new() -> Self {
        let client = openrouter::Client::new(
            &std::env::var("OPENROUTER_API_KEY").unwrap()
        );
        let agent = client.agent("openai/gpt-4o-mini")
            .preamble(
                "You are an insurance claims assistant. \
                 Help users file claims by gathering basic information: \
                 name, brief description of what happened, and contact preference. \
                 Be empathetic and professional."
            )
            .build();
        
        Self { agent }
    }
}

#[async_trait]
impl Task for InitialClaimQueryTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let user_input: String = context.get_sync("user_input").unwrap_or_default();
        
        // Get chat history
        let chat_history = context.get_rig_messages().await;
        
        // Call LLM
        let response = self.agent.chat(&user_input, chat_history).await
            .map_err(|e| GraphError::TaskExecutionFailed(e.to_string()))?;
        
        // Update chat history
        context.add_user_message(user_input).await;
        context.add_assistant_message(response.clone()).await;
        
        // Try to extract structured info
        self.extract_claim_info(&response, &context).await;
        
        Ok(TaskResult::new(Some(response), NextAction::Continue))
    }
}

impl InitialClaimQueryTask {
    fn extract_claim_info(&self, response: &str, context: &Context) {
        // Use LLM to extract structured info from response
        // Store in context for next tasks
    }
}
```

### 2. Insurance Type Classifier

```rust
// insurance-claims-service/src/tasks/insurance_type_classifier.rs

struct InsuranceTypeClassifierTask;

#[async_trait]
impl Task for InsuranceTypeClassifierTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Get claim description from context
        let description: String = context.get_sync("claim_description")
            .unwrap_or_default();
        
        // Use LLM to classify
        let classification = self.classify(&description).await?;
        
        // Store result
        context.set("insurance_type", &classification.insurance_type).await;
        context.set("classification_confidence", classification.confidence).await;
        
        let response = format!(
            "Classified as {} insurance (confidence: {:.0}%)",
            classification.insurance_type,
            classification.confidence * 100.0
        );
        
        Ok(TaskResult::new(Some(response), NextAction::Continue))
    }
}

impl InsuranceTypeClassifierTask {
    async fn classify(&self, description: &str) -> Result<Classification, GraphError> {
        // Call LLM to classify as "car" or "apartment"
        // Parse response and return
    }
}
```

### 3. Detail Collectors

```rust
// Car insurance details
struct CarInsuranceDetailsTask;

#[async_trait]
impl Task for CarInsuranceDetailsTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Gather car-specific details:
        // - Vehicle information
        // - Accident details
        // - Police report info
        // - Damage description
        
        let user_input: String = context.get_sync("user_input").unwrap_or_default();
        let chat_history = context.get_rig_messages().await;
        
        let prompt = format!(
            "Gather car insurance claim details. Ask about: \
             vehicle make/model/year, location of incident, \
             description of damage, police report number if any. \
             User said: {}",
            user_input
        );
        
        let response = self.agent.chat(&prompt, chat_history).await?;
        
        // Extract and store structured data
        context.set("vehicle_info", /* extracted data */).await;
        
        Ok(TaskResult::new(Some(response), NextAction::Continue))
    }
}

// Apartment insurance details
struct ApartmentInsuranceDetailsTask;

#[async_trait]
impl Task for ApartmentInsuranceDetailsTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Gather property-specific details:
        // - Property address
        // - Type of damage
        // - Estimated value
        // - Date/time of incident
    }
}
```

### 4. Smart Claim Validator

```rust
// insurance-claims-service/src/tasks/smart_claim_validator.rs

struct SmartClaimValidatorTask;

#[async_trait]
impl Task for SmartClaimValidatorTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Get claim details
        let claim_amount: f64 = context.get_sync("estimated_amount")
            .unwrap_or(0.0);
        
        // Business rule: Auto-approve claims under $1000
        if claim_amount < 1000.0 {
            context.set("auto_approved", true).await;
            
            return Ok(TaskResult::new(
                Some(format!(
                    "Claim for ${:.2} has been auto-approved!",
                    claim_amount
                )),
                NextAction::Continue
            ));
        }
        
        // High-value claim needs manual review
        context.set("auto_approved", false).await;
        
        Ok(TaskResult::new(
            Some(format!(
                "This claim for ${:.2} requires manager approval. \
                 Please review the details and approve or reject.",
                claim_amount
            )),
            NextAction::WaitForInput  // Pause for human review
        ))
    }
}
```

### 5. Final Summary

```rust
// insurance-claims-service/src/tasks/final_summary.rs

struct FinalSummaryTask;

#[async_trait]
impl Task for FinalSummaryTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        // Gather all data from context
        let insurance_type: String = context.get_sync("insurance_type")
            .unwrap_or_default();
        let auto_approved: bool = context.get_sync("auto_approved")
            .unwrap_or(false);
        let claim_amount: f64 = context.get_sync("estimated_amount")
            .unwrap_or(0.0);
        
        // Generate summary using LLM
        let summary = self.generate_summary(
            &insurance_type,
            auto_approved,
            claim_amount,
        ).await?;
        
        context.set("final_summary", &summary).await;
        
        Ok(TaskResult::new(Some(summary), NextAction::End))
    }
}
```

## Building the Graph

```rust
// insurance-claims-service/src/main.rs

fn build_graph() -> Graph {
    let initial_query = Arc::new(InitialClaimQueryTask::new());
    let classifier = Arc::new(InsuranceTypeClassifierTask::new());
    let car_details = Arc::new(CarInsuranceDetailsTask::new());
    let apartment_details = Arc::new(ApartmentInsuranceDetailsTask::new());
    let validator = Arc::new(SmartClaimValidatorTask);
    let summary = Arc::new(FinalSummaryTask::new());
    
    GraphBuilder::new("insurance_claims")
        .add_task(initial_query.clone())
        .add_task(classifier.clone())
        .add_task(car_details.clone())
        .add_task(apartment_details.clone())
        .add_task(validator.clone())
        .add_task(summary.clone())
        
        // Edge: Initial → Classifier
        .add_edge(initial_query.id(), classifier.id())
        
        // Conditional: Classifier → Car OR Apartment
        .add_conditional_edge(
            classifier.id(),
            |ctx| ctx.get_sync::<String>("insurance_type")
                .map(|t| t == "car")
                .unwrap_or(false),
            car_details.id(),
            apartment_details.id()
        )
        
        // Both lead to validator
        .add_edge(car_details.id(), validator.id())
        .add_edge(apartment_details.id(), validator.id())
        
        // Validator → Summary
        .add_edge(validator.id(), summary.id())
        
        .build()
}
```

## HTTP API

```rust
// Axum HTTP handlers

async fn execute_claim(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ClaimRequest>,
) -> Result<Json<ClaimResponse>, StatusCode> {
    let session_id = request.session_id
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    // Create new session if needed
    if request.session_id.is_none() {
        let session = Session::new_from_task(
            session_id.clone(),
            initial_query.id()
        );
        state.storage.save(session).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    // Set user input
    if let Some(content) = request.content {
        let mut session = state.storage.get(&session_id).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?;
        
        session.context.set("user_input", content).await;
        session.context.set("claim_description", content).await;
        
        state.storage.save(session).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    // Execute workflow
    let result = state.flow_runner.run(&session_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(ClaimResponse {
        session_id,
        response: result.response,
        status: format!("{:?}", result.status),
    }))
}
```

## Testing the Service

```bash
# Start the service
cargo run --bin insurance-claims-service

# File a new claim
curl -X POST http://localhost:3000/execute \
  -H "Content-Type: application/json" \
  -d '{"content": "I need to file a claim for my car accident"}'

# Response:
# {
#   "session_id": "abc-123",
#   "response": "I'm sorry to hear about your accident...",
#   "status": "Paused"
# }

# Continue with more details
curl -X POST http://localhost:3000/execute \
  -H "Content-Type: application/json" \
  -d '{"session_id": "abc-123", "content": "It happened yesterday on Main Street"}'
```

## Key Concepts Demonstrated

1. **LLM Integration**: Natural language understanding
2. **Conditional Routing**: Dynamic paths based on claim type
3. **Business Rules**: Automated decision making
4. **Human-in-the-Loop**: Manual approval workflow
5. **State Management**: Persistent context across interactions

## Next Steps

- [Recommendation Service Example](./recommendation-example.md) - RAG-based system
- [Storage Backends](./storage.md) - Deploy to production
- [LLM Integration](./llm-integration.md) - More AI patterns
