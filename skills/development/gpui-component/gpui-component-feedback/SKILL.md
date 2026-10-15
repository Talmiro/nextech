---
name: gpui-component-feedback
description: >
  Feedback components in GPUI Component library. Use when showing dialogs, alerts,
  notifications, tooltips, or other user feedback elements. Includes Dialog, AlertDialog,
  Alert, Notification, Tooltip, HoverCard, and Popover.
---

## Feedback Components Overview

GPUI Component provides components for user feedback and notifications.

## Dialog

Modal dialog for focused interactions.

```rust
use gpui_component::dialog::*;

Dialog::new("dialog")
    .title("Confirm Action")
    .content("Are you sure you want to proceed?")
    .actions(vec![
        DialogAction::new("Cancel")
            .on_click(|_, _, _| {}),
        DialogAction::new("Confirm")
            .primary()
            .on_click(|_, _, _| {}),
    ])
```

### Usage with Window
```rust
window.open_dialog(dialog, cx);
```

## AlertDialog

Alert confirmation dialog.

```rust
use gpui_component::alert_dialog::*;

AlertDialog::new("alert")
    .title("Delete Item")
    .description("This action cannot be undone.")
    .confirm_label("Delete")
    .cancel_label("Cancel")
    .on_confirm(|_, _, _| println!("Confirmed!"))
```

### Variants
```rust
.alert_info()      // Information
.alert_success()   // Success
.alert_warning()    // Warning
.alert_danger()     // Danger/Error
```

## Alert

Alert/notification banner.

```rust
use gpui_component::alert::*;

Alert::new("alert")
    .variant(AlertVariant::Info)
    .title("Info")
    .description("This is an informational message.")
```

### Variants
```rust
.variant(AlertVariant::Info)
.variant(AlertVariant::Success)
.variant(AlertVariant::Warning)
.variant(AlertVariant::Error)
```

### With Icon
```rust
Alert::new("alert")
    .variant(AlertVariant::Success)
    .icon(Icon::new(IconName::Check))
    .title("Success!")
    .description("Your changes have been saved.")
```

## Notification

Toast notifications.

```rust
use gpui_component::notification::*;

Notification::new("notification")
    .title("Saved")
    .description("Your document has been saved.")
    .show(cx);
```

### Variants
```rust
.notification_info()
.notification_success()
.notification_warning()
.notification_error()
```

## Tooltip

Hover tooltip.

```rust
use gpui_component::tooltip::*;

Button::new("btn")
    .label("Save")
    .tooltip("Save your changes", cx)
```

## HoverCard

Hover card popover.

```rust
use gpui_component::hover_card::*;

HoverCard::new("hover")
    .trigger(trigger_element)
    .content(div().child("Hover card content"))
```

## Popover

Popover container.

```rust
use gpui_component::popover::*;

Popover::new("popover")
    .trigger(trigger_element)
    .content(content_element)
    .placement(PopoverPlacement::Bottom)
```

### Placements
```rust
.placement(PopoverPlacement::Top)
.placement(PopoverPlacement::Bottom)
.placement(PopoverPlacement::Left)
.placement(PopoverPlacement::Right)
```

## Common Patterns

### Success Feedback
```rust
AlertDialog::new("success")
    .alert_success()
    .title("Success!")
    .description("Your action was completed successfully.")
    .on_confirm(|_, _, _| {})
```

### Error Feedback
```rust
AlertDialog::new("error")
    .alert_danger()
    .title("Error")
    .description("Something went wrong. Please try again.")
    .on_confirm(|_, _, _| {})
```

### Warning Confirmation
```rust
AlertDialog::new("warning")
    .alert_warning()
    .title("Confirm Deletion")
    .description("This will permanently delete the item.")
    .confirm_label("Delete")
    .on_confirm(|_, _, _| delete_item())
```
