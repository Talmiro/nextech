---
name: gpui-component-layout
description: >
  Layout components in GPUI Component library. Use when building page layouts,
  panels, sidebars, or organizing content. Includes Sheet, Sidebar, Resizable,
  Collapsible, GroupBox, Scrollable, and more.
---

## Layout Components Overview

GPUI Component provides components for building page layouts and organizing content.

## Sheet

Slide-out panel/drawer.

```rust
use gpui_component::sheet::*;

Sheet::new("sheet")
    .title("Settings")
    .content(div().child("Settings content"))
```

### Positions
```rust
.position(SheetPosition::Left)
.position(SheetPosition::Right)   // Default
.position(SheetPosition::Top)
.position(SheetPosition::Bottom)
```

### Sizes
```rust
.size(SheetSize::Small)   // 320px
.size(SheetSize::Medium)  // 480px
.size(SheetSize::Large)   // 640px
.size(SheetSize::XLarge)  // 800px
```

## Sidebar

Sidebar navigation panel.

```rust
use gpui_component::sidebar::*;

Sidebar::new("sidebar")
    .items(vec![
        SidebarItem::new("home", "Home")
            .icon(Icon::new(IconName::Home)),
        SidebarItem::new("settings", "Settings")
            .icon(Icon::new(IconName::Settings)),
    ])
    .on_select(|id, _, _| println!("Selected: {}", id))
```

### Collapsible
```rust
Sidebar::new("sidebar")
    .collapsible(true)
    .collapsed(false)
```

## Resizable

Resizable panel layout.

```rust
use gpui_component::resizable::*;

Resizable::new("resizable")
    .direction(ResizableDirection::Horizontal)
    .min_size(200)
    .max_size(600)
    .default_size(300)
```

### Directions
```rust
.direction(ResizableDirection::Horizontal)
.direction(ResizableDirection::Vertical)
```

## Collapsible

Collapsible content section.

```rust
use gpui_component::collapsible::*;

Collapsible::new("collapsible")
    .title("Section Title")
    .content(div().child("Hidden content"))
    .open(false)
```

## GroupBox

Group container with title.

```rust
use gpui_component::group_box::*;

GroupBox::new("group")
    .title("Personal Information")
    .content(
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child("Form fields...")
    )
```

## Scrollable

Scroll container.

```rust
use gpui_component::scrollable::*;

Scrollable::new("scroll")
    .content(content_element)
    .direction(ScrollDirection::Vertical)
```

### Directions
```rust
.direction(ScrollDirection::Vertical)    // Default
.direction(ScrollDirection::Horizontal)
.direction(ScrollDirection::Both)
```

## TitleBar

Window title bar.

```rust
use gpui_component::title_bar::*;

TitleBar::new("titlebar")
    .title("My App")
    .controls(vec![
        TitleBarButton::new("minimize")
            .icon(Icon::new(IconName::Minus))
            .on_click(|_, _| {}),
        TitleBarButton::new("close")
            .icon(Icon::new(IconName::X))
            .on_click(|_, _| {}),
    ])
```

## Settings

Settings panel with sections.

```rust
use gpui_component::settings::*;

Settings::new("settings")
    .sections(vec![
        SettingsSection::new("general")
            .title("General")
            .items(vec![
                SettingsItem::new("theme")
                    .title("Theme")
                    .description("Choose your theme")
                    .control(theme_select),
            ]),
    ])
```

## Common Patterns

### Main Layout
```rust
div()
    .flex()
    .size_full()
    .child(
        Sidebar::new("sidebar")
            .width(240)
    )
    .child(
        div()
            .flex_1()
            .p_4()
            .child(MainContent)
    )
```

### Resizable Split
```rust
div()
    .flex()
    .size_full()
    .child(
        Resizable::new("left")
            .direction(ResizableDirection::Horizontal)
            .child(LeftPanel)
    )
    .child(
        Resizable::new("right")
            .direction(ResizableDirection::Horizontal)
            .child(RightPanel)
    )
```
