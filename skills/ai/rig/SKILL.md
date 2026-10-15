---
name: rig
description: >
  Build LLM-powered applications with Rig, the Rust AI framework. Use when creating
  agents, RAG pipelines, tool-calling workflows, structured extraction, or streaming
  completions. Supports 19+ LLM providers (OpenAI, Anthropic, Cohere, Mistral, Gemini,
  Groq, Ollama, DeepSeek, xAI, Together, Perplexity, OpenRouter, HuggingFace, Azure,
  Hyperbolic, Galadriel, Moonshot, Mira, Voyage AI) with a unified provider-agnostic API.
  
  ## Quick Start
  ```rust
  use rig::completion::Prompt;
  use rig::providers::openai;

  #[tokio::main]
  async fn main() -> Result<(), anyhow::Error> {
      let client = openai::Client::from_env();
      let agent = client.agent(openai::GPT_4O).preamble("You are helpful.").build();
      let response = agent.prompt("Hello!").await?;
      println!("{}", response);
      Ok(())
  }
  ```

  ## Core Patterns (see references for full details)
  - **Simple Agent**: `.preamble()`, `.temperature()`, `.max_tokens()`, `.build()`
  - **Agent with Tools**: Implement `Tool` trait, attach via `.tool(MyTool)`
  - **RAG**: `.dynamic_context(n, index)` with vector stores
  - **Streaming**: `agent.stream_prompt()` returning `MultiTurnStreamItem`
  - **Structured Extraction**: `client.extractor::<MyType>()` with schemars/serde
  - **Chat with History**: `agent.chat("msg", history)` maintaining context

  ## Key Rules
  - All async code uses tokio
  - Use `WasmCompatSend`/`WasmCompatSync` instead of `Send`/`Sync` for WASM compatibility
  - Use proper error types with `thiserror` — never `Result<(), String>`
  - Avoid `.unwrap()` — use `?` operator

  ## Reference Files (full details)
  - **tools.md** — Tool trait definition, ToolDefinition struct, ToolEmbedding for RAG-able tools, tool attachment patterns, ToolChoice control
  - **rag.md** — Embed derive macro, EmbeddingsBuilder, VectorStoreIndex trait, in-memory/MongoDB/LanceDB/Qdrant/SQLite/Neo4j/Milvus/SurrealDB backends
  - **providers.md** — All provider initialization (from_env/new), model constants, embedding models per provider
  - **patterns.md** — Multi-turn chat, structured extraction, agent-as-tool, prompt hooks (observability), streaming, chaining, error handling

  ## Commands (opencode slash commands)
  - **rig-provider-checklist** (references/commands/rig-provider-checklist.md) — Implement/review a Rig provider against canonical checklist. Covers provider extension pattern, Capabilities, Builder, model constants, streaming, telemetry, error handling, WASM compat
  - **rig-review-gates** (references/commands/rig-review-gates.md) — Pre-PR review against quality gates. Checks trait bounds, error handling (no String types), no TODOs, comments explain WHY not WHAT, cargo fmt/clippy/test
  - **rig-agent-scaffold** (references/commands/rig-agent-scaffold.md) — Scaffold agent implementation following Rig architecture patterns, builder ergonomics, trait-based abstractions

  ## Additional Skills (specialized workflows)
  - **rig-provider** — Full provider implementation workflow
  - **rig-vector-store** — Vector store backend integration
  - **rig-wasm-check** — WebAssembly compatibility verification
  - **rig-migrate** — Migrate Rig code between versions
  - **rig-review** — Code review focused on Rig patterns
  - **rig-agent-scaffold** — Agent scaffolding workflow

  ## Vector Store Backends
  | Backend | Crate |
  |---------|-------|
  | In-memory | rig-core (built-in) |
  | MongoDB | rig-mongodb |
  | LanceDB | rig-lancedb |
  | Qdrant | rig-qdrant |
  | SQLite | rig-sqlite |
  | Neo4j | rig-neo4j |
  | Milvus | rig-milvus |
  | SurrealDB | rig-surrealdb |

  Full API docs: https://docs.rig.rs
argument-hint: "[what-to-build]"
allowed-tools:
  - Read
  - Glob
  - Grep
  - Edit
  - Write
  - Bash
---
