---
name: gpui-component-review
description: >
  Review GPUI Component usage for consistency with library conventions. Use when reviewing
  code that uses GPUI Component library, checking for proper theming, sizing, and API usage.
---

# GPUI Component Code Review

Use this command to review GPUI Component code for consistency and best practices.

## Review Checklist

### 1. Initialization
- [ ] `gpui_component::init(cx)` called at start of `app.run`
- [ ] `gpui_component_assets::Assets` registered if using icons

### 2. Import Patterns
- [ ] Imports from `gpui_component::{component::*, ...}`
- [ ] Individual component imports for clarity: `use gpui_component::button::*`

### 3. Theming
- [ ] Uses `cx.theme()` for colors (background, foreground, border, etc.)
- [ ] Implements `ActiveTheme` trait when needed
- [ ] Consistent theme usage across components

### 4. Sizing
- [ ] Consistent use of sizes (xsmall, small, medium, large)
- [ ] Default sizes used when appropriate

### 5. Variants
- [ ] Consistent variant usage (primary, danger, ghost, etc.)
- [ ] Proper semantic meaning (danger for destructive actions)

### 6. Component Usage
- [ ] Stateless components: `RenderOnce` pattern
- [ ] Stateful components: `Entity<ComponentState>` pattern
- [ ] Builder pattern used consistently

### 7. Accessibility
- [ ] Proper keyboard navigation
- [ ] Focus management where needed
- [ ] ARIA labels for icons

## Common Issues

### Missing init()
```rust
// Wrong
app.run(move |cx| {
    cx.open_window(...);
});

// Correct
app.run(move |cx| {
    gpui_component::init(cx);
    cx.open_window(...);
});
```

### Wrong Theme Access
```rust
// Wrong
div().bg(gpui::gray(900))

// Correct
div().bg(cx.theme().background)
```

### Inconsistent Sizing
```rust
// Wrong
Button::new("btn").small()
Button::new("btn").large()  // inconsistent

// Correct - pick one size and stick with it
Button::new("btn").small()
Button::new("btn2").small()
```

## Files to Review

- `examples/**/*.rs` - Example code
- `crates/**/src/**/*.rs` - Application code using components
