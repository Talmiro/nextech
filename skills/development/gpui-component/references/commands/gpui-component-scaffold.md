---
name: gpui-component-scaffold
description: >
  Scaffold a new GPUI Component integration or example. Use when the user wants to create
  a new example, demo, or integration using GPUI Component library.
---

# GPUI Component Scaffolding

Use this command to help scaffold a new GPUI Component integration.

## 1. Gather Information

Ask the user for:
- Example name (e.g., "my_form", "data_dashboard")
- Main components to use
- Purpose (demo, learning, production)

## 2. Project Structure

```
examples/
└── {example_name}/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## 3. Cargo.toml Template

```toml
[package]
name = "example-{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
gpui = "{{ GPUI_VERSION }}"
gpui-component = "{{ VERSION }}"
gpui-component-assets = "{{ VERSION }}"
anyhow = "1.0"
```

## 4. Main.rs Template

```rust
use gpui::*;
use gpui_component::{button::*, *};

pub struct ExampleApp;

impl ExampleApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for ExampleApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .size_full()
            .child("Example App")
    }
}

fn main() -> anyhow::Result<()> {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);
    app.run(move |cx| {
        gpui_component::init(cx);
        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|cx| Root::new(cx.new(|_| ExampleApp::new(window, cx)), window, cx))
        });
    });
    Ok(())
}
```

## 5. Component Usage Patterns

### Button
```rust
Button::new("id")
    .primary()
    .label("Click Me")
    .on_click(|_, _, _| println!("Clicked!"))
```

### Input
```rust
Input::new("input-id")
    .placeholder("Enter text...")
    .on_change(|_, value, _, _| println!("Changed: {}", value))
```

### Theme
```rust
use gpui_component::ActiveTheme;

div()
    .bg(cx.theme().background)
    .text_color(cx.theme().foreground)
```

## 6. Run Example

```bash
cargo run --example {example_name}
```
