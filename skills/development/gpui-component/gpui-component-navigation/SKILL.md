---
name: gpui-component-navigation
description: >
  Navigation components in GPUI Component library. Use when building navigation,
  menus, tabs, breadcrumbs, or step indicators. Includes Menu, Tabs, Breadcrumb,
  Stepper, Pagination, Accordion, and more.
---

## Navigation Components Overview

GPUI Component provides components for navigation and user flow.

## Menu

Dropdown menu.

```rust
use gpui_component::menu::*;

Menu::new("menu")
    .trigger(Button::new("trigger").label("Menu"))
    .items(vec![
        MenuItem::new("new")
            .label("New")
            .icon(Icon::new(IconName::FilePlus)),
        MenuItem::new("open")
            .label("Open")
            .icon(Icon::new(IconName::FolderOpen)),
        MenuItem::separator(),
        MenuItem::new("settings")
            .label("Settings")
            .icon(Icon::new(IconName::Settings)),
    ])
    .on_select(|id, _, _| println!("Selected: {}", id))
```

### Submenu
```rust
MenuItem::new("edit")
    .label("Edit")
    .children(vec![
        MenuItem::new("cut").label("Cut"),
        MenuItem::new("copy").label("Copy"),
        MenuItem::new("paste").label("Paste"),
    ])
```

## Tabs

Tab navigation.

```rust
use gpui_component::tabs::*;

Tabs::new("tabs")
    .tabs(vec![
        Tab::new("tab1").label("Overview"),
        Tab::new("tab2").label("Settings"),
        Tab::new("tab3").label("Advanced"),
    ])
    .on_change(|tab_id, _, _| println!("Tab: {}", tab_id))
```

### Tab Variants
```rust
.variant(TabsVariant::Line)      // Default underline
.variant(TabsVariant::Pill)      // Rounded pills
.variant(TabsVariant::Enclosed)  // Bordered enclosed
.variant(TabsVariant::Soft)     // Soft rounded
```

### With Content
```rust
TabsContent::new("tabs")
    .tab("tab1", div().child("Overview content"))
    .tab("tab2", div().child("Settings content"))
```

## Breadcrumb

Breadcrumb navigation.

```rust
use gpui_component::breadcrumb::*;

Breadcrumb::new("breadcrumb")
    .items(vec![
        BreadcrumbItem::new("home", "Home"),
        BreadcrumbItem::new("docs", "Docs"),
        BreadcrumbItem::new("current", "Current Page"),
    ])
    .on_click(|id, _, _| println!("Clicked: {}", id))
```

### With Separator
```rust
Breadcrumb::new("breadcrumb")
    .separator("/")
    .items(items)
```

## Stepper

Step progress indicator.

```rust
use gpui_component::stepper::*;

Stepper::new("stepper")
    .steps(vec![
        Step::new("1").label("Account"),
        Step::new("2").label("Profile"),
        Step::new("3").label("Confirm"),
    ])
    .current_step(1)
```

### States
```rust
Step::new("1").state(StepState::Complete)  // Checkmark
Step::new("2").state(StepState::Current)  // Active
Step::new("3").state(StepState::Upcoming) // Disabled
```

## Pagination

Pagination controls.

```rust
use gpui_component::pagination::*;

Pagination::new("pagination")
    .page(1)
    .per_page(20)
    .total(100)
    .on_change(|page, _, _| println!("Page: {}", page))
```

### Variants
```rust
.variant(PaginationVariant::Default)  // Numbers
.variant(PaginationVariant::Simple)  // Prev/Next only
.variant(PaginationVariant::Brief)   // Compact
```

## Accordion

Accordion/collapsible panels.

```rust
use gpui_component::accordion::*;

Accordion::new("accordion")
    .items(vec![
        AccordionItem::new("item1")
            .title("Section 1")
            .content(div().child("Content 1")),
        AccordionItem::new("item2")
            .title("Section 2")
            .content(div().child("Content 2")),
    ])
```

### Multiple Open
```rust
Accordion::new("accordion")
    .multiple(true)  // Allow multiple open
    .default_open(vec!["item1"])  // Open by default
```

## Rating

Star rating component.

```rust
use gpui_component::rating::*;

Rating::new("rating")
    .value(4)
    .max(5)
    .on_change(|value, _, _| println!("Rating: {}", value))
```

## Toggle

Toggle button.

```rust
use gpui_component::toggle::*;

Toggle::new("toggle")
    .label("Enable feature")
    .on_change(|enabled, _, _| println!("Enabled: {}", enabled))
```

## Calendar

Calendar date picker.

```rust
use gpui_component::calendar::*;

Calendar::new("calendar")
    .selected(date)
    .on_change(|date, _, _| println!("Selected: {:?}", date))
```

## DatePicker

Date selection input.

```rust
use gpui_component::date_picker::*;

DatePicker::new("date")
    .placeholder("Select date")
    .on_change(|date, _, _| println!("Date: {:?}", date))
```

## Common Patterns

### Page Navigation
```rust
div()
    .flex()
    .flex_col()
    .size_full()
    .child(Breadcrumb::new("breadcrumb").items(breadcrumbs))
    .child(Tabs::new("tabs").tabs(tabs))
    .child(
        div()
            .flex_1()
            .p_4()
            .child(TabContent)
    )
    .child(Pagination::new("pagination").total(total_pages))
```

### Wizard Flow
```rust
div()
    .flex()
    .flex_col()
    .gap_4()
    .child(Stepper::new("stepper").steps(steps).current_step(current))
    .child(StepContent)
    .child(
        div()
            .flex()
            .justify_between()
            .child(BackButton)
            .child(NextButton)
    )
```
