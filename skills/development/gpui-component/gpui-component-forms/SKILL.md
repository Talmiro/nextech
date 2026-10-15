---
name: gpui-component-forms
description: >
  Form components in GPUI Component library. Use when working with form elements,
  user inputs, validation, or interactive form controls. Includes Button, Input,
  Select, Checkbox, Radio, Switch, Slider, NumberInput, and more.
---

## Form Components Overview

GPUI Component provides a comprehensive set of form components for building interactive forms.

## Button

```rust
use gpui_component::button::*;

Button::new("submit")
    .primary()
    .label("Submit")
    .on_click(|_, _, _| println!("Clicked!"))
```

### Variants
```rust
.primary()   // Primary action
.secondary() // Secondary action
.danger()     // Destructive action
.warning()    // Warning action
.success()    // Success action
.ghost()      // Subtle appearance
.outline()    // Bordered
```

### Sizes
```rust
.xsmall()     // Extra small
.small()      // Small
.medium()     // Default
.large()      // Large
```

## Input

```rust
use gpui_component::input::*;

Input::new("email")
    .placeholder("Enter email...")
    .on_change(|text, _, _, _| println!("{}", text))
```

### With Validation
```rust
Input::new("email")
    .placeholder("Enter email...")
    .error("Invalid email format")
    .validate(|text| text.contains('@'))
```

## Select

```rust
use gpui_component::select::*;

let options = vec![
    SelectOption::new("1", "Option 1"),
    SelectOption::new("2", "Option 2"),
];

Select::new("select", options)
    .on_change(|value, _, _, _| println!("Selected: {}", value))
```

## Checkbox

```rust
use gpui_component::checkbox::*;

Checkbox::new("agree")
    .label("I agree to terms")
    .on_change(|checked, _, _, _| println!("Checked: {}", checked))
```

## Radio

```rust
use gpui_component::radio::*;

RadioGroup::new("options")
    .option("1", "Option 1")
    .option("2", "Option 2")
    .on_change(|value, _, _, _| println!("Selected: {}", value))
```

## Switch

```rust
use gpui_component::switch::*;

Switch::new("toggle")
    .label("Enable feature")
    .on_change(|enabled, _, _, _| println!("Enabled: {}", enabled))
```

## Slider

```rust
use gpui_component::slider::*;

Slider::new("volume")
    .min(0)
    .max(100)
    .value(50)
    .on_change(|value, _, _, _| println!("Volume: {}", value))
```

## NumberInput

```rust
use gpui_component::number_input::*;

NumberInput::new("quantity")
    .min(1)
    .max(100)
    .step(1)
    .on_change(|value, _, _, _| println!("Quantity: {}", value))
```

## OtpInput

```rust
use gpui_component::otp_input::*;

OtpInput::new("otp")
    .length(6)
    .on_complete(|code, _, _, _| println!("Code: {}", code))
```

## ColorPicker

```rust
use gpui_component::color_picker::*;

ColorPicker::new("color")
    .value("#ff0000")
    .on_change(|color, _, _, _| println!("Color: {}", color))
```

## Common Patterns

### Form Layout
```rust
div()
    .flex()
    .flex_col()
    .gap_4()
    .child(Input::new("name").label("Name"))
    .child(Input::new("email").label("Email"))
    .child(Button::new("submit").primary().label("Submit"))
```

### Disabled State
```rust
Button::new("btn")
    .disabled(true)
    .label("Disabled")
```
