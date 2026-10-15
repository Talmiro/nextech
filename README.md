# Skills Package

Comprehensive collection of specialized skills for AI-assisted software development and operations.

## Overview

| Metric | Value |
|--------|-------|
| Total Categories | 16 |
| Total Skills | 85 |

## Architecture

This package follows a **hierarchical parent-child skill structure**:

```
skills/
├── skills/
│   ├── ai/              → Parent skill + sub-skills
│   ├── architecture/    → Parent skill + sub-skills
│   ├── cloud/           → Parent skill + sub-skills
│   └── ...
├── microsoft-foundry/   → Standalone (not in hierarchy)
└── rust/                → Standalone in development/
```

### How It Works

1. **Parent Skills** act as routers - they read the user's request and route to the appropriate sub-skill
2. **Sub-skills** contain the detailed instructions for specific tasks
3. This structure saves context tokens by loading only what's needed

## Categories

### 🎯 ai
LLM frameworks and AI agent orchestration.

- **rig** - Build LLM-powered applications with Rust
- **graph-flow** - Multi-agent workflow systems in Rust

### 🏗️ architecture
Software architecture and system design.

- coupling-analysis
- domain-analysis
- component-identification-sizing
- component-common-domain-detection
- component-flattening-analysis
- domain-identification-grouping
- legacy-migration-planner
- decomposition-planning-roadmap
- frontend-blueprint
- react-composition-patterns

### ☁️ cloud
Cloud deployment and infrastructure.

- aws-advisor
- vercel-deploy
- netlify-deploy
- cloudflare-deploy
- render-deploy

### ✨ creation
Code and document creation.

- skill-architect
- subagent-creator
- cursor-subagent-creator
- create-rfc
- create-adr
- create-technical-design-doc

### 🎲 decision-making
Decision making frameworks.

- the-fool

### 🎨 design
UI/UX design implementation.

- figma
- figma-implement-design
- web-design-guidelines
- frontend-design

### 💻 development
Software development assistance.

- codenavi
- coding-guidelines
- confluence-assistant
- docs-writer
- gh-address-comments
- jira-assistant
- nestjs-modular-monolith
- react-native-expert
- rust
- shopify-developer
- tauri
- tlc-spec-driven
- gpui
- gpui-component

### 📈 gtm
Go-to-Market strategies and execution.

- ai-sdr
- ai-cold-outreach
- ai-pricing
- ai-ugc-ads
- ai-seo
- content-to-pipeline
- expansion-retention
- gtm-engineering
- gtm-metrics
- lead-enrichment
- multi-platform-launch
- partner-affiliate
- paid-creative-ai
- positioning-icp
- sales-motion-design
- social-selling
- solo-founder-gtm
- video-outreach

### 📚 learning
Learning and skill development.

- learning-opportunities

### 💬 messaging
Messaging protocols and systems.

- zenoh
- zmq

### 📊 monitoring
Application monitoring.

- sentry

### 👁️ observability
AI observability and tracing.

- langfuse-rs

### ⚡ performance
Web performance optimization.

- core-web-vitals
- perf-astro
- perf-lighthouse
- perf-web-optimization

### ✅ quality
Code quality and best practices.

- web-quality-audit
- web-accessibility
- react-best-practices
- seo
- web-best-practices

### 🔒 security
Security analysis and best practices.

- security-threat-model
- security-best-practices
- security-ownership-map
- sast

### 🛠️ tooling
Development tools and utilities.

- chrome-devtools
- excalidraw-studio
- gh-fix-ci
- mermaid-studio
- nx-ci-monitor
- nx-generate
- nx-run-tasks
- nx-workspace

## Standalone Skills

These skills are not in the hierarchy:

| Skill | Description |
|-------|-------------|
| microsoft-foundry | Azure AI Foundry deployment and management |
| rust | Comprehensive Rust coding guidelines (179 rules) |

## Usage

When the AI assistant needs to use a skill:

1. **Parent skill is loaded first** - provides routing guidance
2. **Sub-skill is loaded on-demand** - only when specific task matches

### Example

User asks: "Analyze the coupling between my modules"

1. Load `architecture/SKILL.md` (parent)
2. See routing → use `coupling-analysis/SKILL.md`
3. Execute detailed instructions

## Meta.json Format

Each skill has a `meta.json` with:

```json
{
  "name": "skill-name",
  "description": "What this skill does",
  "category": "parent-category",
  "tags": ["relevant", "keywords"],
  "version": "1.0.0"
}
```

## Contributing

When adding new skills:

1. Create the skill directory under appropriate category
2. Add `SKILL.md` with frontmatter (name, description)
3. Add `meta.json` with name matching directory
4. Update parent category's `SKILL.md` and `meta.json`

## Version

Current version: 1.0.0

Last updated: 2026-03-29
