# 📚 UI Retained API Reference

The Lunaris retained-mode UI system for building complex, stateful editor interfaces.

---

## Overview

```rust
use lunaris_editor::ui_retained::*;

// Create UI tree with theme
let mut ui = UiTree::new(Theme::dark());

// Create widgets
let panel = ui.create_panel("Main Panel");
let button = ui.create_button(panel, "Click Me");

// Handle events
ui.on_click(button, |ctx| {
    println!("Button clicked!");
});

// Update and render
ui.update(delta_time);
ui.render(&mut renderer);
```

---

## UiTree

The root container for all UI widgets.

### Creation

```rust
// Dark theme (default)
let ui = UiTree::new(Theme::dark());

// Light theme
let ui = UiTree::new(Theme::light());

// Custom theme
let ui = UiTree::new(my_custom_theme);
```

### Core Methods

| Method | Description |
|--------|-------------|
| `create_widget(parent, type)` | Create widget under parent |
| `remove_widget(id)` | Remove widget and children |
| `get_widget(id)` | Get widget reference |
| `update(dt)` | Update all widgets |
| `render(renderer)` | Render all widgets |
| `handle_event(event)` | Process input event |

### Example

```rust
let mut ui = UiTree::new(Theme::dark());

// Create hierarchy
let root = ui.root();
let sidebar = ui.create_widget(root, WidgetType::Panel);
let content = ui.create_widget(root, WidgetType::Panel);

// Configure layout
ui.set_layout(root, Layout::horizontal());
ui.set_size(sidebar, Size::fixed(250.0, Size::fill()));
ui.set_size(content, Size::fill_both());
```

---

## Widget Types

### Panel

Container for other widgets.

```rust
let panel = ui.create_widget(parent, WidgetType::Panel);

// Styling
ui.set_background(panel, Colors::bg_subtle());
ui.set_padding(panel, Spacing::all(16.0));
ui.set_border(panel, Border::all(1.0, Colors::border_default()));
ui.set_corner_radius(panel, 8.0);
```

### Button

Interactive clickable element.

```rust
let button = ui.create_widget(parent, WidgetType::Button {
    label: "Save".into(),
    icon: Some("save".into()),
    variant: ButtonVariant::Primary,
});

ui.on_click(button, |_| {
    save_document();
});
```

**Variants:**
- `Primary` - Accent color, high emphasis
- `Secondary` - Muted background
- `Outline` - Border only
- `Ghost` - No background
- `Danger` - Red, destructive actions

### Label

Text display.

```rust
let label = ui.create_widget(parent, WidgetType::Label {
    text: "Hello World".into(),
    size: FontSize::Base,
    weight: FontWeight::Normal,
});
```

### TextInput

Single-line text entry.

```rust
let input = ui.create_widget(parent, WidgetType::TextInput {
    value: String::new(),
    placeholder: "Enter name...".into(),
    max_length: Some(50),
});

ui.on_change(input, |value| {
    println!("New value: {}", value);
});
```

### Slider

Numeric value with drag.

```rust
let slider = ui.create_widget(parent, WidgetType::Slider {
    value: 0.5,
    min: 0.0,
    max: 1.0,
    step: 0.01,
});

ui.on_change(slider, |value| {
    set_volume(value);
});
```

### Checkbox

Boolean toggle.

```rust
let checkbox = ui.create_widget(parent, WidgetType::Checkbox {
    checked: false,
    label: Some("Enable feature".into()),
});
```

### Dropdown

Selection from options.

```rust
let dropdown = ui.create_widget(parent, WidgetType::Dropdown {
    options: vec!["Option A", "Option B", "Option C"],
    selected: 0,
});
```

### TreeView

Hierarchical list.

```rust
let tree = ui.create_widget(parent, WidgetType::TreeView);

// Add items
ui.tree_add_item(tree, "Root", None);
ui.tree_add_item(tree, "Child 1", Some("Root"));
ui.tree_add_item(tree, "Child 2", Some("Root"));
```

### ScrollView

Scrollable container.

```rust
let scroll = ui.create_widget(parent, WidgetType::ScrollView {
    horizontal: false,
    vertical: true,
});

// Add content
let content = ui.create_widget(scroll, WidgetType::Panel);
```

---

## Layout System

### Direction

```rust
// Horizontal layout (row)
ui.set_layout(widget, Layout::horizontal());

// Vertical layout (column)
ui.set_layout(widget, Layout::vertical());

// Stack (overlapping)
ui.set_layout(widget, Layout::stack());

// Grid
ui.set_layout(widget, Layout::grid(3, 2)); // 3 cols, 2 rows
```

### Sizing

```rust
// Fixed size
ui.set_size(widget, Size::fixed(200.0, 100.0));

// Fill available space
ui.set_size(widget, Size::fill_both());

// Fill width, fixed height
ui.set_size(widget, Size::new(Size::fill(), Size::fixed(50.0)));

// Percentage
ui.set_size(widget, Size::percent(50.0, 100.0));
```

### Spacing

```rust
// Padding (inside)
ui.set_padding(widget, Spacing::all(16.0));
ui.set_padding(widget, Spacing::symmetric(16.0, 8.0)); // h, v
ui.set_padding(widget, Spacing::new(16, 8, 16, 8)); // l, t, r, b

// Margin (outside)
ui.set_margin(widget, Spacing::all(8.0));

// Gap between children
ui.set_gap(widget, 8.0);
```

### Alignment

```rust
// Main axis alignment
ui.set_justify(widget, Justify::Start);    // flex-start
ui.set_justify(widget, Justify::Center);   // center
ui.set_justify(widget, Justify::End);      // flex-end
ui.set_justify(widget, Justify::Between);  // space-between

// Cross axis alignment
ui.set_align(widget, Align::Start);
ui.set_align(widget, Align::Center);
ui.set_align(widget, Align::Stretch);
```

---

## Styling

### Colors

```rust
// Backgrounds
ui.set_background(widget, Colors::bg_base());
ui.set_background(widget, Colors::bg_subtle());
ui.set_background(widget, Colors::accent_emphasis());

// Text
ui.set_text_color(widget, Colors::fg_default());
ui.set_text_color(widget, Colors::fg_muted());

// Custom
ui.set_background(widget, Color::hex("#1a1a2e"));
ui.set_background(widget, Color::rgba(26, 26, 46, 255));
```

### Borders

```rust
// All sides
ui.set_border(widget, Border::all(1.0, Colors::border_default()));

// Specific sides
ui.set_border(widget, Border {
    top: Some((1.0, Colors::border_default())),
    bottom: Some((1.0, Colors::border_default())),
    left: None,
    right: None,
});

// Rounded corners
ui.set_corner_radius(widget, 8.0);
ui.set_corner_radius(widget, CornerRadius::new(8, 8, 0, 0)); // tl, tr, br, bl
```

### Shadows

```rust
ui.set_shadow(widget, Shadow::sm());
ui.set_shadow(widget, Shadow::md());
ui.set_shadow(widget, Shadow::lg());
ui.set_shadow(widget, Shadow::glow(Colors::accent_emphasis()));
```

---

## Events

### Click Events

```rust
ui.on_click(widget, |ctx| {
    // Handle click
});

ui.on_double_click(widget, |ctx| {
    // Handle double-click
});

ui.on_right_click(widget, |ctx| {
    // Show context menu
});
```

### Hover Events

```rust
ui.on_hover_enter(widget, |ctx| {
    show_tooltip("Info");
});

ui.on_hover_exit(widget, |ctx| {
    hide_tooltip();
});
```

### Value Changes

```rust
ui.on_change(input_widget, |new_value| {
    // Value changed
});
```

### Drag & Drop

```rust
// Make draggable
ui.set_draggable(widget, true);

ui.on_drag_start(widget, |ctx| {
    ctx.set_drag_data(my_data);
});

ui.on_drop(target, |ctx, data| {
    // Handle drop
});
```

### Focus

```rust
ui.on_focus(widget, |ctx| {
    // Widget focused
});

ui.on_blur(widget, |ctx| {
    // Widget lost focus
});
```

---

## Animation

### Transitions

```rust
// Enable smooth transitions
ui.set_transition(widget, Transition {
    property: "background",
    duration: 0.2,
    easing: Easing::EaseOut,
});

// Multiple properties
ui.set_transitions(widget, vec![
    Transition::background(0.2),
    Transition::transform(0.3),
    Transition::opacity(0.15),
]);
```

### Animate Values

```rust
// Animate a property
ui.animate(widget, Animation {
    property: Property::Opacity,
    from: 0.0,
    to: 1.0,
    duration: 0.3,
    easing: Easing::EaseInOut,
});

// Sequence animations
ui.animate_sequence(widget, vec![
    Animation::scale(1.0, 1.1, 0.1),
    Animation::scale(1.1, 1.0, 0.1),
]);
```

---

## Theme

### Using Themes

```rust
// Built-in themes
let theme = Theme::dark();
let theme = Theme::light();

// Apply to tree
ui.set_theme(theme);
```

### Custom Theme

```rust
let theme = Theme {
    colors: ThemeColors {
        bg_base: Color::hex("#0a0a0f"),
        bg_subtle: Color::hex("#12121a"),
        fg_default: Color::hex("#ffffff"),
        accent: Color::hex("#6366f1"),
        // ...
    },
    fonts: ThemeFonts {
        family: "Inter".into(),
        family_mono: "JetBrains Mono".into(),
        size_base: 14.0,
        // ...
    },
    metrics: ThemeMetrics {
        spacing_base: 4.0,
        border_radius: 6.0,
        // ...
    },
};
```

---

## Complete Example

```rust
use lunaris_editor::ui_retained::*;

fn create_settings_panel(ui: &mut UiTree) -> WidgetId {
    let panel = ui.create_widget(ui.root(), WidgetType::Panel);
    ui.set_padding(panel, Spacing::all(24.0));
    ui.set_layout(panel, Layout::vertical());
    ui.set_gap(panel, 16.0);
    
    // Title
    let title = ui.create_widget(panel, WidgetType::Label {
        text: "Settings".into(),
        size: FontSize::XL,
        weight: FontWeight::Bold,
    });
    
    // Volume slider
    let volume_row = ui.create_widget(panel, WidgetType::Panel);
    ui.set_layout(volume_row, Layout::horizontal());
    ui.set_align(volume_row, Align::Center);
    ui.set_gap(volume_row, 12.0);
    
    ui.create_widget(volume_row, WidgetType::Label {
        text: "Volume".into(),
        size: FontSize::Base,
        weight: FontWeight::Normal,
    });
    
    let slider = ui.create_widget(volume_row, WidgetType::Slider {
        value: 0.8,
        min: 0.0,
        max: 1.0,
        step: 0.05,
    });
    ui.set_size(slider, Size::new(Size::fill(), Size::fixed(24.0)));
    
    // Checkbox
    ui.create_widget(panel, WidgetType::Checkbox {
        checked: true,
        label: Some("Enable notifications".into()),
    });
    
    // Save button
    let btn = ui.create_widget(panel, WidgetType::Button {
        label: "Save Settings".into(),
        icon: Some("save".into()),
        variant: ButtonVariant::Primary,
    });
    
    ui.on_click(btn, |_| {
        save_settings();
    });
    
    panel
}
```

---

## Best Practices

1. **Reuse widget IDs** - Store IDs for widgets you'll update
2. **Batch updates** - Group changes before calling `update()`
3. **Use themes** - Don't hardcode colors
4. **Event cleanup** - Remove handlers when widgets are destroyed
5. **Layout efficiency** - Prefer `Layout::vertical/horizontal` over absolute positioning

---

## See Also

- [Design System](./design_system.md)
- [Widgets Reference](./widgets.md)
- [Theme Customization](./theming.md)
