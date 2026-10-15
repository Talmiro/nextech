---
name: gpui-new-component
description: >
  Scaffold a new GPUI component following project patterns, builder ergonomics, and trait-based abstractions.
  Use when the user wants to create a new UI component in crates/ui.
---

# GPUI Component Scaffolding

Use this command to help scaffold a new GPUI component. Follow these steps:

## 1. Gather Information

Ask the user for:
- Component name (e.g., "Badge", "Tooltip", "Avatar")
- Component type (stateless, stateful, or composite)
- Key features/variants needed

## 2. Create Component File

Create in `crates/ui/src/` following the style guide:

```rust
// crates/ui/src/{component_name}.rs
use gpui::{
    div, prelude::FluentBuilder as _,
    AnyElement, App, Div, ElementId, InteractiveElement,
    IntoElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, StyleRefinement, Styled, Window,
};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct ComponentName {
    id: ElementId,
    base: Div,
    style: StyleRefinement,
    // ... fields
}

impl ComponentName {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            style: StyleRefinement::default(),
            // ... init
        }
    }
}

impl InteractiveElement for ComponentName {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for ComponentName {}

impl Styled for ComponentName {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for ComponentName {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        self.base
    }
}
```

## 3. Register Module

Add to `crates/ui/src/lib.rs`:
```rust
pub mod component_name;
```

## 4. Create Story

Create `crates/story/src/stories/{component_name}_story.rs`:
```rust
use crate::{Story, StoryContainer};
use gpui::{App, ElementId, Entity, Render, Window};

pub struct ComponentNameStory;

impl Story for ComponentNameStory {
    fn title() -> &'static str {
        "ComponentName"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}
```

## 5. Register Story

Add to `crates/story/src/stories/mod.rs` and `crates/story/src/main.rs`.

## 6. Generate Documentation

Create documentation following the style guide patterns.
