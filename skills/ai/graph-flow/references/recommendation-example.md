# Recommendation Service Example

This example demonstrates building a RAG (Retrieval Augmented Generation) based recommendation system using graph-flow.

## Overview

The recommendation service shows:
- Vector search integration for semantic matching
- Multi-step reasoning with context accumulation
- Structured data extraction from unstructured input
- Query refinement for better results

## Workflow Structure

```
QueryInput → QueryRefinement
                 │
                 ▼
         VectorSearch
                 │
                 ▼
       ProcessResults
                 │
                 ▼
       AnswerGeneration
                 │
                 ▼
            FinalAnswer
```

## Task Definitions

### 1. Query Input

```rust
// recommendation-service/src/tasks/query_input.rs

struct QueryInputTask;

#[async_trait]
impl Task for QueryInputTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let user_input: String = context.get_sync("user_input")
            .unwrap_or_default();
        
        if user_input.is_empty() {
            return Ok(TaskResult::new(
                Some("What would you like recommendations for?".to_string()),
                NextAction::WaitForInput
            ));
        }
        
        // Store raw query
        context.set("raw_query", user_input.clone()).await;
        
        Ok(TaskResult::new(
            Some(format!("Looking for: {}", user_input)),
            NextAction::Continue
        ))
    }
}
```

### 2. Query Refinement

```rust
// recommendation-service/src/tasks/query_refinement.rs

use rig::{providers::openrouter, agent::Agent};

struct QueryRefinementTask {
    agent: impl Agent,
}

impl QueryRefinementTask {
    fn new() -> Self {
        let client = openrouter::Client::new(
            &std::env::var("OPENROUTER_API_KEY").unwrap()
        );
        let agent = client.agent("openai/gpt-4o-mini")
            .preamble(
                "You are a query optimization assistant. \
                 Refine user queries to improve search results. \
                 Expand abbreviations, add relevant context, \
                 and structure for vector search."
            )
            .build();
        
        Self { agent }
    }
}

#[async_trait]
impl Task for QueryRefinementTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let raw_query: String = context.get_sync("raw_query")
            .unwrap_or_default();
        
        // Refine query using LLM
        let refined = self.refine_query(&raw_query).await?;
        
        context.set("refined_query", refined.clone()).await;
        
        Ok(TaskResult::new(
            Some(format!("Refined query: {}", refined)),
            NextAction::Continue
        ))
    }
}

impl QueryRefinementTask {
    async fn refine_query(&self, query: &str) -> Result<String, GraphError> {
        let prompt = format!(
            "Refine this search query for better vector search results: '{}'\n\
             Return ONLY the refined query, nothing else.",
            query
        );
        
        let response = self.agent.completion(&prompt).await
            .map_err(|e| GraphError::TaskExecutionFailed(e.to_string()))?;
        
        Ok(response.trim().to_string())
    }
}
```

### 3. Vector Search

```rust
// recommendation-service/src/tasks/vector_search.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub content: String,
    pub score: f64,
    pub metadata: serde_json::Value,
}

struct VectorSearchTask {
    // Vector store client (e.g., qdrant, pinecone, etc.)
}

impl VectorSearchTask {
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Task for VectorSearchTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let refined_query: String = context.get_sync("refined_query")
            .unwrap_or_default();
        
        // Perform vector search
        let results = self.search(&refined_query, 5).await?;
        
        // Store results in context
        context.set("search_results", &results).await;
        
        // Create summary
        let summary = results.iter()
            .map(|r| r.title.clone())
            .collect::<Vec<_>>()
            .join(", ");
        
        Ok(TaskResult::new(
            Some(format!("Found {} results: {}", results.len(), summary)),
            NextAction::Continue
        ))
    }
}

impl VectorSearchTask {
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>, GraphError> {
        // Example: Use qdrant or other vector store
        // 
        // let results = self.client.search(
        //     CollectionName::from("recommendations"),
        //     query,
        //     limit,
        // ).await?;
        //
        // Ok(results)
        
        // For demo, return mock results
        Ok(vec![
            SearchResult {
                id: "1".to_string(),
                title: "Product A".to_string(),
                content: "Description of Product A".to_string(),
                score: 0.95,
                metadata: serde_json::json!({}),
            },
            SearchResult {
                id: "2".to_string(),
                title: "Product B".to_string(),
                content: "Description of Product B".to_string(),
                score: 0.87,
                metadata: serde_json::json!({}),
            },
        ])
    }
}
```

### 4. Process Results

```rust
// recommendation-service/src/tasks/delivery.rs

#[derive(Debug, Serialize, Deserialize)]
struct ProcessedRecommendation {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub match_score: f64,
    pub reasoning: String,
}

struct ProcessResultsTask;

#[async_trait]
impl Task for ProcessResultsTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let results: Vec<SearchResult> = context.get_sync("search_results")
            .unwrap_or_default();
        
        let processed: Vec<ProcessedRecommendation> = results
            .into_iter()
            .map(|r| {
                ProcessedRecommendation {
                    id: r.id,
                    title: r.title.clone(),
                    summary: r.content.chars().take(100).collect(),
                    match_score: r.score,
                    reasoning: format!("Matched with {:.0}% confidence", r.score * 100.0),
                }
            })
            .collect();
        
        context.set("processed_recommendations", &processed).await;
        
        let count = processed.len();
        Ok(TaskResult::new(
            Some(format!("Processed {} recommendations", count)),
            NextAction::Continue
        ))
    }
}
```

### 5. Answer Generation

```rust
// recommendation-service/src/tasks/answer_generation.rs

use rig::{providers::openrouter, agent::Agent};

struct AnswerGenerationTask {
    agent: impl Agent,
}

impl AnswerGenerationTask {
    fn new() -> Self {
        let client = openrouter::Client::new(
            &std::env::var("OPENROUTER_API_KEY").unwrap()
        );
        let agent = client.agent("openai/gpt-4o-mini")
            .preamble(
                "You are a helpful recommendation assistant. \
                 Based on the user's query and search results, \
                 provide personalized recommendations with explanations."
            )
            .build();
        
        Self { agent }
    }
}

#[async_trait]
impl Task for AnswerGenerationTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let query: String = context.get_sync("raw_query").unwrap_or_default();
        let recommendations: Vec<ProcessedRecommendation> = context
            .get_sync("processed_recommendations")
            .unwrap_or_default();
        
        // Generate natural language response
        let answer = self.generate_answer(&query, &recommendations).await?;
        
        context.set("final_answer", answer.clone()).await;
        
        Ok(TaskResult::new(Some(answer), NextAction::End))
    }
}

impl AnswerGenerationTask {
    async fn generate_answer(
        &self,
        query: &str,
        recommendations: &[ProcessedRecommendation],
    ) -> Result<String, GraphError> {
        let recs_text = recommendations
            .iter()
            .map(|r| format!("- {}: {}\n  Why: {}\n", r.title, r.summary, r.reasoning))
            .collect::<Vec<_>>()
            .join("\n");
        
        let prompt = format!(
            "User asked: '{}'\n\n\
             Based on the following recommendations:\n{}\n\n\
             Provide a friendly, helpful response explaining these recommendations.",
            query,
            recs_text
        );
        
        let response = self.agent.completion(&prompt).await
            .map_err(|e| GraphError::TaskExecutionFailed(e.to_string()))?;
        
        Ok(response)
    }
}
```

## Building the Graph

```rust
// recommendation-service/src/main.rs

fn build_recommendation_graph() -> Graph {
    let query_input = Arc::new(QueryInputTask);
    let query_refinement = Arc::new(QueryRefinementTask::new());
    let vector_search = Arc::new(VectorSearchTask::new());
    let process_results = Arc::new(ProcessResultsTask);
    let answer_generation = Arc::new(AnswerGenerationTask::new());
    
    GraphBuilder::new("recommendation_workflow")
        .add_task(query_input.clone())
        .add_task(query_refinement.clone())
        .add_task(vector_search.clone())
        .add_task(process_results.clone())
        .add_task(answer_generation.clone())
        
        // Linear flow
        .add_edge(query_input.id(), query_refinement.id())
        .add_edge(query_refinement.id(), vector_search.id())
        .add_edge(vector_search.id(), process_results.id())
        .add_edge(process_results.id(), answer_generation.id())
        
        .build()
}
```

## HTTP API

```rust
async fn get_recommendations(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RecommendationRequest>,
) -> Result<Json<RecommendationResponse>, StatusCode> {
    let session_id = request.session_id
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    
    // Create session on first request
    if request.session_id.is_none() {
        let session = Session::new_from_task(
            session_id.clone(),
            query_input.id()
        );
        session.context.set("user_input", &request.query).await;
        state.storage.save(session).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    
    // Execute workflow
    let result = state.flow_runner.run(&session_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(RecommendationResponse {
        session_id,
        answer: result.response,
        status: format!("{:?}", result.status),
    }))
}
```

## Testing the Service

```bash
# Start the service
cargo run --bin recommendation-service

# Get recommendations
curl -X POST http://localhost:3000/recommend \
  -H "Content-Type: application/json" \
  -d '{"query": "What books would you recommend for learning programming?"}'

# Response:
# {
#   "session_id": "xyz-789",
#   "answer": "Based on your interest in programming, here are some recommendations...",
#   "status": "Completed"
# }
```

## Advanced: Adding Feedback Loop

```rust
// Allow users to provide feedback and refine recommendations

struct FeedbackTask;

#[async_trait]
impl Task for FeedbackTask {
    async fn run(&self, context: Context) -> graph_flow::Result<TaskResult> {
        let feedback: String = context.get_sync("user_feedback")
            .unwrap_or_default();
        
        if feedback.to_lowercase().contains("not good") {
            // Refine search with feedback
            context.set("refine_with_feedback", true).await;
            
            return Ok(TaskResult::new(
                Some("Let me find better recommendations for you.".to_string()),
                NextAction::GoTo("QueryRefinementTask")
            ));
        }
        
        Ok(TaskResult::new(
            Some("Thank you for your feedback!".to_string()),
            NextAction::End
        ))
    }
}

// Add feedback edge
GraphBuilder::new("recommendation_with_feedback")
    // ... existing tasks ...
    .add_task(Arc::new(FeedbackTask))
    .add_edge(answer_generation.id(), feedback.id())
    .add_conditional_edge(
        feedback.id(),
        |ctx| ctx.get_sync::<bool>("refine_with_feedback")
            .unwrap_or(false),
        query_refinement.id(),  // Loop back
        "end"  // Actually need a terminal task
    )
    .build()
```

## Key Concepts Demonstrated

1. **Vector Search**: Semantic matching with embeddings
2. **Query Refinement**: LLM-powered query optimization
3. **Multi-step Processing**: Results processing pipeline
4. **RAG Pattern**: Retrieval Augmented Generation
5. **Context Accumulation**: Building up data through workflow

## Next Steps

- Add more sophisticated ranking algorithms
- Implement hybrid search (keyword + vector)
- Add user preference learning
- Scale to multiple recommendation categories
- [Storage Backends](./storage.md) - Deploy to production
