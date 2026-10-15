# GPUI Component Style Guide

Style guide for using GPUI Component library consistently.

## Initialization

Always call `gpui_component::init(cx)` at the start of `app.run`:

```rust
fn main() -> anyhow::Result<()> {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);
    app.run(move |cx| {
        gpui_component::init(cx);  // Always first!
        // ... rest of code
    });
}
```

## Import Patterns

### Module Imports
```rust
use gpui_component::{button::*, *};
use gpui_component::dialog::*;
```

### Specific Component Imports
```rust
use gpui_component::{Button, Dialog, Select};
```

## Theming

### Using ActiveTheme

```rust
use gpui_component::ActiveTheme;

impl Render for MyView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .border_1()
            .border_color(cx.theme().border)
    }
}
```

### Available Theme Colors

```rust
cx.theme().background      // Page background
cx.theme().foreground     // Primary text
cx.theme().border          // Borders
cx.theme().primary         // Primary accent
cx.theme().secondary       // Secondary accent
cx.theme().destructive     // Error/danger
cx.theme().muted           // Muted/disabled
cx.theme().muted_foreground
cx.theme().card            // Card background
cx.theme().card_foreground
cx.theme().popover         // Popover background
cx.theme().popover_foreground
cx.theme().input           // Input background
cx.theme().input_border
cx.theme().ring            // Focus ring
```

## Sizing

Components support consistent sizes:

```rust
Button::new("btn").xsmall()   // Extra small
Button::new("btn").small()    // Small
Button::new("btn").medium()  // Default
Button::new("btn").large()   // Large
```

## Variants

### Button Variants
```rust
Button::new("btn").primary()   // Primary action
Button::new("btn").secondary() // Secondary action
Button::new("btn").danger()   // Destructive action
Button::new("btn").warning()  // Warning action
Button::new("btn").success()  // Success action
Button::new("btn").ghost()    // Subtle/secondary
Button::new("btn").outline()   // Bordered
```

### Usage Guidelines
- `primary()`: Main action in a view
- `danger()`: Destructive actions (delete, remove)
- `ghost()`: Low-emphasis actions, secondary buttons
- `outline()`: Medium emphasis, standalone buttons

## Builder Pattern

All components use the builder pattern:

```rust
Button::new("id")
    .primary()
    .label("Submit")
    .small()
    .on_click(|_, _, _| { /* handler */ })
```

### Method Chaining
```rust
Component::new("id")
    .size_variant()
    .style_variant()
    .property("value")
    .event_handler()
```

## Component Categories

### Form Components
```rust
Button::new("btn")     // Interactive button
Input::new("input")    // Text input
Select::new("select")  // Dropdown selection
Checkbox::new("cb")    // Boolean checkbox
Radio::new("radio")    // Radio option
Switch::new("switch")  // Toggle switch
Slider::new("slider")  // Range slider
```

### Data Display
```rust
Table::new(state)       // Data table (stateful)
List::new(state)        // Virtualized list (stateful)
Badge::new("badge")     // Count/status badge
Avatar::new("avatar")   // User avatar
Tag::new("tag")         // Label tag
```

### Feedback
```rust
Dialog::new("dialog")  // Modal dialog
Alert::new("alert")    // Alert banner
Notification::new()    // Toast notification
Tooltip::new("tip")   // Hover tooltip
```

### Layout
```rust
Sheet::new("sheet")     // Slide-out panel
Sidebar::new("sidebar") // Sidebar navigation
Tabs::new("tabs")      // Tab container
Resizable::new()        // Resizable panels
```

## Icon Usage

```rust
use gpui_component::{Icon, IconName, ActiveTheme};

Icon::new(IconName::Check)
    .size(IconSize::Small)
    .color(cx.theme().foreground)

// With tooltip
Icon::new(IconName::Settings)
    .tooltip("Settings", cx)
```

## Event Handlers

### Button Click
```rust
Button::new("btn")
    .label("Click Me")
    .on_click(|event, window, cx| {
        // event: &ClickEvent
        // window: &mut Window
        // cx: &mut Context<Self>
    })
```

### Input Change
```rust
Input::new("input")
    .on_change(|text, _, _, _| {
        println!("Input changed: {}", text);
    })
```

## Quick Checklist

- [ ] `gpui_component::init(cx)` called at start
- [ ] Imports follow pattern: `use gpui_component::{component::*, ...}`
- [ ] Theme colors via `cx.theme()`
- [ ] Consistent sizing across components
- [ ] Semantic variants (primary for main, danger for destructive)
- [ ] Builder pattern for configuration
- [ ] Icons from `gpui_component_assets` if needed
