---
name: gpui-component
description: >
  Build desktop UI applications with GPUI Component, a comprehensive Rust component library
  for GPUI. Use when creating UI components, using pre-built components (Button, Dialog,
  Table, Select, etc.), managing forms, or building desktop applications.

  ## Quick Start
  ```rust
  use gpui::*;
  use gpui_component::{button::*, *};

  pub struct MyApp;

  impl Render for MyApp {
      fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
          div()
              .flex()
              .gap_4()
              .items_center()
              .child("Hello!")
              .child(
                  Button::new("ok")
                      .primary()
                      .label("Let's Go!")
                      .on_click(|_, _, _| println!("Clicked!")),
              )
      }
  }

  fn main() {
      let app = Application::new();
      app.run(move |cx| {
          gpui_component::init(cx);
          cx.open_window(WindowOptions::default(), |window, cx| {
              cx.new(|cx| Root::new(cx.new(|_| MyApp), window, cx))
          });
      });
  }
  ```

  ## Core Patterns (see references for full details)
  - **Components**: Import from `gpui_component::{component::*, ...}`, use builder pattern
  - **Theming**: Use `ActiveTheme` trait, access colors via `cx.theme()`
  - **Sizing**: Components support xs, sm, md, lg sizes via builder methods
  - **Variants**: Components offer visual variants (primary, danger, ghost, etc.)
  - **Stateful Components**: Use Entity<ComponentState> pattern (Dropdown, List, Table)
  - **Icons**: Import icons via `gpui_component_assets`, use `Icon::new(IconName::Xxx)`

  ## Reference Files (full details)
  - **gpui-component-style-guide.md** — Component library style guide, API conventions, theming
  - **docs/** — Complete documentation (getting-started, theming, components, examples)
  - **docs/getting-started.md** — Setup, installation, basic usage
  - **docs/components/** — Individual component documentation (button, input, dialog, table, etc.)
  - **examples/** — Working examples (hello_world, dialog, input, system_monitor, etc.)

  ## Commands (opencode slash commands)
  - **gpui-component-scaffold** (references/commands/gpui-component-scaffold.md) — Scaffold a new component example or integration
  - **gpui-component-review** (references/commands/gpui-component-review.md) — Review GPUI Component usage for consistency
  - **gpui-component-pr-description** (references/commands/gpui-component-pr-description.md) — Generate GitHub PR description

  ## Additional Skills (specialized workflows)
  - **gpui-component-forms** — Form components (Input, Select, Checkbox, Radio, etc.)
  - **gpui-component-data-display** — Data display (Table, List, Badge, Avatar, etc.)
  - **gpui-component-feedback** — Feedback components (Dialog, Alert, Notification, etc.)
  - **gpui-component-layout** — Layout components (Sheet, Sidebar, Resizable, etc.)
  - **gpui-component-navigation** — Navigation components (Menu, Tabs, Breadcrumb, etc.)

  ## Documentation & Examples
  - **docs/** — Full component documentation with usage examples
  - **docs/getting-started.md** — Installation and first app guide
  - **docs/components/index.md** — 60+ component docs (button, dialog, table, etc.)
  - **examples/** — Runnable examples (hello_world, input, dialog_overlay, etc.)

  ## Component Categories

  ### Form Components
  | Component | Description |
  |-----------|-------------|
  | Button | Interactive button with variants |
  | Input | Text input with validation |
  | Select | Dropdown selection |
  | Checkbox | Boolean selection |
  | Radio | Single selection from options |
  | Switch | Toggle switch |
  | Slider | Range slider |
  | NumberInput | Numeric input with stepper |
  | OtpInput | One-time password input |
  | ColorPicker | Color selection |

  ### Data Display
  | Component | Description |
  |-----------|-------------|
  | Table | Sortable, selectable data table |
  | List | Virtualized list component |
  | Badge | Status/count badge |
  | Avatar | User avatar display |
  | Tag | Label/tag element |
  | Kbd | Keyboard key display |
  | Skeleton | Loading placeholder |
  | Spinner | Loading spinner |
  | Progress | Progress bar |
  | Chart | Chart visualization |
  | Plot | Plot visualization |
  | Image | Image display |

  ### Feedback
  | Component | Description |
  |-----------|-------------|
  | Dialog | Modal dialog |
  | AlertDialog | Alert confirmation dialog |
  | Alert | Alert/notification banner |
  | Notification | Toast notifications |
  | Tooltip | Hover tooltip |
  | HoverCard | Hover card popover |
  | Popover | Popover container |

  ### Layout
  | Component | Description |
  |-----------|-------------|
  | Sheet | Slide-out panel |
  | Sidebar | Sidebar navigation |
  | Resizable | Resizable panel layout |
  | Scrollable | Scroll container |
  | Collapsible | Collapsible content |
  | GroupBox | Group container |

  ### Navigation
  | Component | Description |
  |-----------|-------------|
  | Menu | Dropdown menu |
  | Tabs | Tab navigation |
  | Breadcrumb | Breadcrumb navigation |
  | Stepper | Step progress indicator |
  | Pagination | Pagination controls |

  ### Overlay
  | Component | Description |
  |-----------|-------------|
  | TitleBar | Window title bar |
  | Settings | Settings panel |
  | Calendar | Calendar date picker |
  | DatePicker | Date selection |
  | Rating | Star rating |
  | Toggle | Toggle button |
  | Accordion | Accordion/collapsible panels |

  
argument-hint: "[what-to-build]"
allowed-tools:
  - Read
  - Glob
  - Grep
  - Edit
  - Write
  - Bash
---
