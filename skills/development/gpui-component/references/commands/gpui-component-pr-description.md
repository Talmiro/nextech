---
name: gpui-component-pr-description
description: >
  Generate GitHub Pull Request description for GPUI Component changes. Use when the user
  wants to create a PR description for component implementations, examples, or integrations.
---

# GPUI Component PR Description Generator

Use this command to generate a PR description for GPUI Component changes.

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
- [ ] Example created (if applicable)
- [ ] Documentation updated (if applicable)
- [ ] Tests added (if applicable)
- [ ] `cargo fmt` and `cargo clippy` pass
```

## Quick PR Description

For simple component additions:
```markdown
## Summary
Added `ComponentName` component to gpui-component

## Features
- Feature 1
- Feature 2

## Usage
```rust
ComponentName::new("id")
    .variant()
    .on_event(|_| println!("Event!"))
```
```

## Review Areas
- API design consistency
- Theming usage
- Component composition patterns
- Example quality
- Documentation completeness
