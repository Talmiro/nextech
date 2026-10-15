---
name: quality
description: >
  Code quality and best practices. Use for audits, accessibility, SEO, and best practices.

  This is a parent skill that routes to specific sub-skills based on user needs.
---

# Quality Skill

This is a **parent skill** that acts as a router to specialized sub-skills.

## Routing

Use the appropriate sub-skill based on the user's request:

| Sub-Skill | When to Use |
|-----------|-------------|
| [web-quality-audit](./web-quality-audit/SKILL.md) | Web quality auditing |
| [web-accessibility](./web-accessibility/SKILL.md) | Web accessibility (WCAG) compliance |
| [react-best-practices](./react-best-practices/SKILL.md) | React best practices |
| [seo](./seo/SKILL.md) | Search engine optimization |
| [web-best-practices](./web-best-practices/SKILL.md) | General web best practices |

## Usage

1. Read the user's request
2. Identify which sub-skill matches the need
3. Use the specific sub-skill for detailed guidance

**Do NOT** try to handle requests that require specialized sub-skills. Route to the appropriate sub-skill instead.
