---
name: security
description: >
  Security analysis and best practices.

  This is a parent skill that routes to specific sub-skills based on user needs.
---

# Security Skill

This is a **parent skill** that acts as a router to specialized sub-skills.

## Routing

Use the appropriate sub-skill based on the user's request:

| Sub-Skill | When to Use |
|-----------|-------------|
| [security-threat-model](./security-threat-model/SKILL.md) | Threat modeling and analysis |
| [security-best-practices](./security-best-practices/SKILL.md) | Security best practices |
| [security-ownership-map](./security-ownership-map/SKILL.md) | Security ownership mapping |
| [sast](./sast/SKILL.md) | Static Application Security Testing |

## Usage

1. Read the user's request
2. Identify which sub-skill matches the need
3. Use the specific sub-skill for detailed guidance

**Do NOT** try to handle requests that require specialized sub-skills. Route to the appropriate sub-skill instead.
