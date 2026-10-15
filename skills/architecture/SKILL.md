---
name: architecture
description: >
  Software architecture and system design. Use for analyzing code structure, component relationships, domain modeling, coupling analysis, and architectural patterns.

  This is a parent skill that routes to specific sub-skills based on user needs.
---

# Architecture Skill

This is a **parent skill** that acts as a router to specialized sub-skills.

## Routing

Use the appropriate sub-skill based on the user's request:

| Sub-Skill | When to Use |
|-----------|-------------|
| [coupling-analysis](./coupling-analysis/SKILL.md) | Analyze coupling between modules using three-dimensional model (strength, distance, volatility) |
| [domain-analysis](./domain-analysis/SKILL.md) | Identify and analyze domain boundaries and business capabilities |
| [component-identification-sizing](./component-identification-sizing/SKILL.md) | Identify components and determine appropriate sizing |
| [component-common-domain-detection](./component-common-domain-detection/SKILL.md) | Detect common domains across components |
| [component-flattening-analysis](./component-flattening-analysis/SKILL.md) | Analyze and flatten component hierarchies |
| [domain-identification-grouping](./domain-identification-grouping/SKILL.md) | Group related domains together |
| [legacy-migration-planner](./legacy-migration-planner/SKILL.md) | Plan migration from legacy systems |
| [decomposition-planning-roadmap](./decomposition-planning-roadmap/SKILL.md) | Plan decomposition of monoliths |
| [frontend-blueprint](./frontend-blueprint/SKILL.md) | Design frontend architecture |
| [react-composition-patterns](./react-composition-patterns/SKILL.md) | React component composition patterns |

## Usage

1. Read the user's request
2. Identify which sub-skill matches the need
3. Use the specific sub-skill for detailed guidance

**Do NOT** try to handle requests that require specialized sub-skills. Route to the appropriate sub-skill instead.
