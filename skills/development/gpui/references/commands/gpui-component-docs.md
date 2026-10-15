---
name: gpui-component-docs
description: >
  Generate documentation for GPUI components. Use when writing docs, documenting components, or creating component documentation.
---

# GPUI Component Documentation

Use this command to generate documentation for GPUI components.

## Documentation Pattern

```rust
/// A brief description of the component.
///
/// Extended description if needed, explaining the purpose and usage.
#[derive(IntoElement)]
pub struct ComponentName {
    // fields
}

impl ComponentName {
    /// Create a new ComponentName with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self { }

    /// Set the label text.
    ///
    /// # Arguments
    /// * `label` - The text to display
    pub fn label(mut self, label: impl Into<Text>) -> Self { }

    /// Set the click handler.
    ///
    /// The `&ClickEvent` parameter contains information about the click event.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self { }
}
```

## Key Documentation Rules

1. **Struct docs**: Brief description + extended if needed
2. **Method docs**: Start with action verb ("Set", "Create", "Add")
3. **Complex params**: Use `# Arguments` section
4. **Callbacks**: Document what parameters mean
5. **Return values**: If not `Self`, document

## Checklist

- [ ] Document struct with `///`
- [ ] Document each public method
- [ ] Use `# Arguments` for multi-parameter methods
- [ ] Document callback parameters
- [ ] Keep docs concise but complete
