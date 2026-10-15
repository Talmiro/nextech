---
name: gpui-pr-description
description: >
  Generate GitHub Pull Request description for GPUI component changes. Use when the user wants to create a PR description for component implementations.
---

# GPUI PR Description Generator

Use this command to generate a PR description for GPUI component changes.

## PR Template

```markdown
## Summary
<!-- Brief description of the change -->

## Changes
<!-- Detailed list of changes -->

## Motivation
<!-- Why this change was needed -->

## Testing
<!-- How was this tested -->

## Screenshots (if applicable)
<!-- Before/after or new component screenshots -->

## Checklist
- [ ] Component follows style guide
- [ ] Story created and registered
- [ ] Documentation complete
- [ ] Tests added (if applicable)
```

## Quick PR Description

For simple component additions:
```markdown
## Summary
Added `ComponentName` component to crates/ui

## Features
- List of features

## Usage
```rust
// Code example
```
```

## Review Areas
- Code style consistency
- API design
- Accessibility
- Testing coverage
