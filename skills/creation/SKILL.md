---
name: creation
description: >
  Code and document creation. Use for generating code, technical documents, RFCs, and ADRs.

  This is a parent skill that routes to specific sub-skills based on user needs.
---

# Creation Skill

This is a **parent skill** that acts as a router to specialized sub-skills.

## Routing

Use the appropriate sub-skill based on the user's request:

| Sub-Skill | When to Use |
|-----------|-------------|
| [skill-architect](./skill-architect/SKILL.md) | Design and architect new skills |
| [subagent-creator](./subagent-creator/SKILL.md) | Create AI sub-agents |
| [cursor-subagent-creator](./cursor-subagent-creator/SKILL.md) | Create Cursor sub-agents |
| [create-rfc](./create-rfc/SKILL.md) | Create Request for Comments (RFC) documents |
| [create-adr](./create-adr/SKILL.md) | Create Architecture Decision Records (ADR) |
| [create-technical-design-doc](./create-technical-design-doc/SKILL.md) | Create technical design documents |

## Usage

1. Read the user's request
2. Identify which sub-skill matches the need
3. Use the specific sub-skill for detailed guidance

**Do NOT** try to handle requests that require specialized sub-skills. Route to the appropriate sub-skill instead.
