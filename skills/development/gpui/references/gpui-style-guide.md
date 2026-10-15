# GPUI Style Guide

Code style guide derived from gpui-component implementation patterns.

**Based on**: Analysis of Button, Checkbox, Input, Select, and other components in crates/ui

## Component Structure

### Basic Component Pattern

```rust
use gpui::{
    div, prelude::FluentBuilder as _, AnyElement, App, Div, ElementId,
    InteractiveElement, IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, StyleRefinement, Styled, Window,
};

#[derive(IntoElement)]
pub struct MyComponent {
    id: ElementId,
    base: Div,
    style: StyleRefinement,
    size: Size,
    disabled: bool,
    selected: bool,
    label: Option<Text>,
    children: Vec<AnyElement>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}
```

## Field Organization

```rust
pub struct Component {
    // 1. Identity
    id: ElementId,
    base: Div,
    style: StyleRefinement,

    // 2. Configuration
    size: Size,
    disabled: bool,
    selected: bool,

    // 3. Content/children
    label: Option<Text>,
    children: Vec<AnyElement>,

    // 4. Callbacks
    on_click: Option<Rc<dyn Fn(Args, &mut Window, &mut App) + 'static>>,
}
```

## Import Organization

```rust
// 1. External crate imports
use std::rc::Rc;

// 2. Crate imports
use crate::{ActiveTheme, Disableable, Icon, IconName, Sizable};

// 3. GPUI imports
use gpui::{
    div, prelude::FluentBuilder as _, px,
    AnyElement, App, Div, ElementId, InteractiveElement,
    IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, StyleRefinement, Styled, Window,
};
```

## Variant Patterns

### Enum Variants

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ButtonVariant {
    Primary,
    #[default]
    Secondary,
    Danger,
    Ghost,
}
```

### Trait-Based Variant API

```rust
pub trait ButtonVariants: Sized {
    fn with_variant(self, variant: ButtonVariant) -> Self;
    fn primary(self) -> Self { self.with_variant(ButtonVariant::Primary) }
    fn danger(self) -> Self { self.with_variant(ButtonVariant::Danger) }
}
```

## Common Traits

### Sizable

```rust
impl Sizable for MyComponent {
    fn with_size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }
}
```

### Selectable

```rust
impl Selectable for MyComponent {
    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
    fn is_selected(&self) -> bool { self.selected }
}
```

### Disableable

```rust
impl Disableable for MyComponent {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    fn is_disabled(&self) -> bool { self.disabled }
}
```

## Documentation Pattern

```rust
/// A brief description.
#[derive(IntoElement)]
pub struct ComponentName { }

impl ComponentName {
    /// Create a new ComponentName.
    pub fn new(id: impl Into<ElementId>) -> Self { }

    /// Set the label.
    ///
    /// # Arguments
    /// * `label` - The text to display
    pub fn label(mut self, label: impl Into<Text>) -> Self { }
}
```

## Quick Checklist

- [ ] `#[derive(IntoElement)]` on struct
- [ ] Include `id`, `base`, `style` fields
- [ ] Implement `InteractiveElement`, `Styled`
- [ ] Implement `RenderOnce`
- [ ] Use `Rc<dyn Fn>` for callbacks
- [ ] Import `prelude::FluentBuilder as _`
- [ ] Use theme colors via `cx.theme()`
