---
name: gpui-component-data-display
description: >
  Data display components in GPUI Component library. Use when displaying data,
  tables, lists, badges, avatars, or visual data representations. Includes Table,
  List, VirtualList, Badge, Avatar, Tag, Chart, and more.
---

## Data Display Components Overview

GPUI Component provides components for displaying data in various formats.

## Table

High-performance sortable and selectable data table.

```rust
use gpui_component::table::*;

pub struct TableState {
    columns: Vec<Column>,
    rows: Vec<Row>,
}

let table = Table::new(state)
    .columns(vec![
        Column::new("name", "Name"),
        Column::new("email", "Email"),
    ])
    .on_sort(|column, order, _, _| /* handle sort */)
    .on_select(|row, _, _| /* handle select */)
```

### Features
- Column sorting
- Row selection
- Pagination
- Custom cell rendering

## List

Virtualized list for large datasets.

```rust
use gpui_component::list::*;

List::new(state)
    .items(items)
    .render_item(|item, index, _| {
        div().child(item.title.clone())
    })
```

## VirtualList

High-performance virtualized list for thousands of items.

```rust
use gpui_component::virtual_list::*;

VirtualList::new(state)
    .item_height(48)
    .items(items)
    .on_click(|index, _, _| /* handle click */)
```

## Badge

Status or count badge.

```rust
use gpui_component::badge::*;

Badge::new("count")
    .label("5")
    .variant(BadgeVariant::Default)

Badge::new("status")
    .label("Active")
    .variant(BadgeVariant::Success)
```

### Variants
```rust
.badge_default()   // Default gray
.badge_success()   // Green success
.badge_warning()   // Yellow warning
.badge_danger()    // Red danger
.badge_outline()   // Outlined
```

## Avatar

User avatar display.

```rust
use gpui_component::avatar::*;

Avatar::new("user")
    .src("https://example.com/avatar.jpg")
    .fallback("JD")  // Initials fallback
    .size(AvatarSize::Medium)
```

### Sizes
```rust
.xsmall()  // 24px
.small()   // 32px
.medium()  // 40px
.large()   // 64px
.xlarge()  // 96px
```

## Tag

Label/tag element.

```rust
use gpui_component::tag::*;

Tag::new("tag")
    .label("Tag Label")
    .on_remove(|_, _, _| println!("Removed!"))
```

## Skeleton

Loading placeholder.

```rust
use gpui_component::skeleton::*;

Skeleton::new("skeleton")
    .width("100%")
    .height("20px")
    .animate(true)
```

## Spinner

Loading spinner.

```rust
use gpui_component::spinner::*;

Spinner::new("loading")
    .size(SpinnerSize::Medium)
```

## Progress

Progress bar.

```rust
use gpui_component::progress::*;

Progress::new("progress")
    .value(75)
    .max(100)
    .show_label(true)
```

## Chart

Chart visualization.

```rust
use gpui_component::chart::*;

Chart::new("chart")
    .chart_type(ChartType::Bar)
    .data(data)
```

## Plot

Plot visualization.

```rust
use gpui_component::plot::*;

Plot::new("plot")
    .series(series)
    .x_axis(XAxis::new())
    .y_axis(YAxis::new())
```

## Image

Image display.

```rust
use gpui_component::image::*;

Image::new("image")
    .src("https://example.com/image.jpg")
    .alt("Description")
    .fit(ImageFit::Cover)
```
