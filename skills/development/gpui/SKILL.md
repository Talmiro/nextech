---
name: gpui
description: >
  Build UI applications with GPUI, Zed's native GUI framework. Use when creating
  UI components, managing state, handling events, implementing layouts, or building
  desktop applications in Rust.
  
  ## Quick Start
  ```rust
  use gpui::{div, App, Render, Window};

  pub struct MyApp;

  impl Render for MyApp {
      fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
          div()
              .flex()
              .bg(gpui::gray(900))
              .text_color(gpui::white())
              .child("Hello, GPUI!")
      }
  }

  fn main() {
      App::new().run(|cx: &mut App| {
          cx.open_window(WindowOptions::default(), |window, cx| {
              cx.new(|cx| MyApp)
          });
      });
  }
  ```

  ## Core Patterns (see references for full details)
  - **Components**: Implement `Render` or `RenderOnce` traits, use `IntoElement`
  - **State Management**: Use `Entity<T>` with `Context<T>` for reactive state
  - **Events**: Emit events with `cx.emit()`, subscribe with `cx.subscribe()`
  - **Layout**: Flexbox-like layout with `.flex()`, `.gap()`, `.items_()`
  - **Styling**: Chain style methods, use `Styled` trait for custom components
  - **Async**: Use `cx.spawn()` for async operations with entity access

  ## Reference Files (full details)
  - **gpui-style-guide.md** — Component project style guide, naming conventions, patterns
  - **gpui-layout-and-style.md** — Layout systems, styling properties, flexbox
  - **gpui-context.md** — App, Window, AsyncApp contexts, entity operations
  - **gpui-entity.md** — Entity management, state handling, reactive patterns
  - **gpui-event.md** — Custom events, observers, event subscriptions
  - **gpui-action.md** — Action definitions, keyboard shortcuts, key bindings
  - **gpui-focus-handle.md** — Focus management, keyboard navigation
  - **gpui-async.md** — Async operations, spawn, background tasks
  - **gpui-element.md** — Low-level Element API for custom UI components

  ## Commands (opencode slash commands)
  - **gpui-new-component** (references/commands/gpui-new-component.md) — Scaffold new GPUI component following project patterns, builder ergonomics, trait-based abstractions
  - **gpui-component-story** (references/commands/gpui-component-story.md) — Create story examples for components
  - **gpui-component-docs** (references/commands/gpui-component-docs.md) — Generate documentation for components
  - **gpui-pr-description** (references/commands/gpui-pr-description.md) — Generate GitHub PR description

  ## Additional Skills (specialized workflows)
  - **new-component** — Create new GPUI components from scratch
  - **generate-component-story** — Write story examples for components
  - **generate-component-documentation** — Generate component documentation
  - **gpui-layout-and-style** — Layout and styling techniques
  - **gpui-context** — Context management patterns
  - **gpui-entity** — Entity and state management
  - **gpui-event** — Event handling and subscriptions
  - **gpui-action** — Actions and keyboard shortcuts
  - **gpui-focus-handle** — Focus management
  - **gpui-async** — Async operations and background tasks
  - **gpui-element** — Low-level Element API
  - **gpui-global** — Global state management
  - **gpui-test** — Testing GPUI applications
  - **gpui-style-guide** — Project style guide

  Full API docs: https://docs.gpui.rs
argument-hint: "[what-to-build]"
allowed-tools:
  - Read
  - Glob
  - Grep
  - Edit
  - Write
  - Bash
---
