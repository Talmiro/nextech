---
name: messaging
description: >
  Messaging protocols and systems. Use for pub/sub, message queues, and distributed communication.

  This is a parent skill that routes to specific sub-skills based on user needs.
---

# Messaging Skill

This is a **parent skill** that acts as a router to specialized sub-skills.

## Routing

Use the appropriate sub-skill based on the user's request:

| Sub-Skill | When to Use |
|-----------|-------------|
| [zenoh](./zenoh/SKILL.md) | See skill documentation |
| [zmq](./zmq/SKILL.md) | See skill documentation |

## Usage

1. Read the user's request
2. Identify which sub-skill matches the need
3. Use the specific sub-skill for detailed guidance
