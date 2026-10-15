---
name: gpui-component-story
description: >
  Create story examples for GPUI components. Use when the user wants to write stories, create examples, or demonstrate component usage.
---

# GPUI Component Story Creation

Use this command to create story examples for GPUI components.

## Story Location

Stories are in `crates/story/src/stories/`

## Basic Story Template

```rust
use crate::{Story, StoryContainer};
use gpui::{App, Entity, Render, Window};

pub struct ComponentNameStory;

impl ComponentNameStory {
    fn view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        cx.new(|cx| ComponentNameStory)
    }
}

impl Story for ComponentNameStory {
    fn title() -> &'static str {
        "ComponentName"
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl Render for ComponentNameStory {
    fn render(&mut self, window: &mut Window, cx: &mut App) -> impl gpui::IntoElement {
        use gpui::prelude::FluentBuilder as _;

        gpui::div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            .child("Story content here")
    }
}
```

## Registration

1. Add to `crates/story/src/stories/mod.rs`:
```rust
mod component_name_story;
pub use component_name_story::ComponentNameStory;
```

2. Add to `crates/story/src/main.rs`:
```rust
StoryContainer::panel::<ComponentNameStory>(window, cx),
```

## Best Practices

- One story per file
- Title should match component name
- Show different variants/states
- Include interactive examples
