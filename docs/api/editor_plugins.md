# 🔌 Creating Editor Plugins with ui_retained

Learn how to extend the Lunaris Editor with custom panels, tools, and widgets.

---

## Overview

Editor plugins use the `ui_retained` system to create custom UI that integrates seamlessly with the editor. You can create:

- **Custom Panels** - New dockable windows
- **Custom Inspectors** - Property editors for your components
- **Custom Tools** - Viewport tools and gizmos
- **Custom Widgets** - Reusable UI components

---

## Quick Start: Your First Plugin

### 1. Create Plugin Structure

```rust
// my_plugin/mod.rs
use lunaris_editor::prelude::*;

pub struct MyPlugin;

impl EditorPlugin for MyPlugin {
    fn name(&self) -> &str {
        "My Custom Plugin"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    fn register(&self, editor: &mut EditorContext) {
        // Register your panels, tools, widgets
        editor.register_panel::<MyCustomPanel>();
        editor.register_inspector::<MyComponent, MyComponentInspector>();
    }
}

// Export the plugin
#[no_mangle]
pub fn create_plugin() -> Box<dyn EditorPlugin> {
    Box::new(MyPlugin)
}
```

### 2. Create a Custom Panel

```rust
pub struct MyCustomPanel {
    // Your panel state
    items: Vec<String>,
    selected: Option<usize>,
    search: String,
}

impl EditorPanel for MyCustomPanel {
    fn id(&self) -> &str {
        "my_custom_panel"
    }

    fn title(&self) -> &str {
        "My Panel"
    }

    fn icon(&self) -> &str {
        "custom_icon"
    }

    fn build(&mut self, ui: &mut UiTree, ctx: &EditorContext) -> WidgetId {
        let panel = ui.create_widget(ui.root(), WidgetType::Panel);
        ui.set_layout(panel, Layout::vertical());
        ui.set_padding(panel, Spacing::all(16.0));
        ui.set_gap(panel, 8.0);

        // Header with title and search
        let header = self.build_header(ui, panel);

        // List of items
        let list = self.build_list(ui, panel);

        // Action buttons
        let actions = self.build_actions(ui, panel);

        panel
    }

    fn update(&mut self, ctx: &EditorContext, dt: f32) {
        // Update logic each frame
    }
}

impl MyCustomPanel {
    fn build_header(&mut self, ui: &mut UiTree, parent: WidgetId) -> WidgetId {
        let header = ui.create_widget(parent, WidgetType::Panel);
        ui.set_layout(header, Layout::horizontal());
        ui.set_gap(header, 8.0);

        // Search input
        let search_input = ui.create_widget(header, WidgetType::TextInput {
            value: self.search.clone(),
            placeholder: "Search...".into(),
            max_length: None,
        });
        ui.set_size(search_input, Size::fill_width());

        ui.on_change(search_input, |new_value| {
            self.search = new_value;
        });

        header
    }

    fn build_list(&mut self, ui: &mut UiTree, parent: WidgetId) -> WidgetId {
        let scroll = ui.create_widget(parent, WidgetType::ScrollView {
            horizontal: false,
            vertical: true,
        });
        ui.set_size(scroll, Size::fill_both());

        let list = ui.create_widget(scroll, WidgetType::Panel);
        ui.set_layout(list, Layout::vertical());
        ui.set_gap(list, 4.0);

        for (i, item) in self.items.iter().enumerate() {
            if !self.search.is_empty() && !item.contains(&self.search) {
                continue;
            }

            let row = ui.create_widget(list, WidgetType::Button {
                label: item.clone(),
                icon: None,
                variant: if Some(i) == self.selected {
                    ButtonVariant::Primary
                } else {
                    ButtonVariant::Ghost
                },
            });

            let idx = i;
            ui.on_click(row, move |_| {
                self.selected = Some(idx);
            });
        }

        scroll
    }

    fn build_actions(&mut self, ui: &mut UiTree, parent: WidgetId) -> WidgetId {
        let actions = ui.create_widget(parent, WidgetType::Panel);
        ui.set_layout(actions, Layout::horizontal());
        ui.set_justify(actions, Justify::End);
        ui.set_gap(actions, 8.0);

        let add_btn = ui.create_widget(actions, WidgetType::Button {
            label: "Add".into(),
            icon: Some("plus".into()),
            variant: ButtonVariant::Primary,
        });

        ui.on_click(add_btn, |_| {
            self.items.push(format!("Item {}", self.items.len()));
        });

        let remove_btn = ui.create_widget(actions, WidgetType::Button {
            label: "Remove".into(),
            icon: Some("minus".into()),
            variant: ButtonVariant::Danger,
        });

        ui.on_click(remove_btn, |_| {
            if let Some(idx) = self.selected {
                self.items.remove(idx);
                self.selected = None;
            }
        });

        actions
    }
}
```

---

## Custom Inspectors

Create custom property editors for your components:

```rust
pub struct MyComponentInspector;

impl ComponentInspector for MyComponentInspector {
    type Component = MyComponent;

    fn build(&self, ui: &mut UiTree, parent: WidgetId, component: &mut MyComponent) {
        // Health slider
        let health_row = ui.create_widget(parent, WidgetType::Panel);
        ui.set_layout(health_row, Layout::horizontal());
        ui.set_align(health_row, Align::Center);
        ui.set_gap(health_row, 8.0);

        ui.create_widget(health_row, WidgetType::Label {
            text: "Health".into(),
            size: FontSize::Base,
            weight: FontWeight::Normal,
        });

        let slider = ui.create_widget(health_row, WidgetType::Slider {
            value: component.health,
            min: 0.0,
            max: 100.0,
            step: 1.0,
        });

        ui.on_change(slider, |value| {
            component.health = value;
        });

        // Speed input
        let speed_row = ui.create_widget(parent, WidgetType::Panel);
        ui.set_layout(speed_row, Layout::horizontal());

        ui.create_widget(speed_row, WidgetType::Label {
            text: "Speed".into(),
            size: FontSize::Base,
            weight: FontWeight::Normal,
        });

        let input = ui.create_widget(speed_row, WidgetType::NumberInput {
            value: component.speed,
            min: Some(0.0),
            max: Some(50.0),
            step: 0.5,
        });

        ui.on_change(input, |value| {
            component.speed = value;
        });

        // Team dropdown
        let team_row = ui.create_widget(parent, WidgetType::Panel);
        ui.set_layout(team_row, Layout::horizontal());

        ui.create_widget(team_row, WidgetType::Label {
            text: "Team".into(),
            size: FontSize::Base,
            weight: FontWeight::Normal,
        });

        let dropdown = ui.create_widget(team_row, WidgetType::Dropdown {
            options: vec!["Red", "Blue", "Green", "Yellow"],
            selected: component.team as usize,
        });

        ui.on_change(dropdown, |index| {
            component.team = Team::from_index(index);
        });
    }
}
```

---

## Custom Widgets

Create reusable widget components:

```rust
/// Color picker with preview
pub struct ColorPickerWidget {
    pub color: [f32; 4],
    pub show_alpha: bool,
}

impl ColorPickerWidget {
    pub fn build(&mut self, ui: &mut UiTree, parent: WidgetId) -> WidgetId {
        let container = ui.create_widget(parent, WidgetType::Panel);
        ui.set_layout(container, Layout::vertical());
        ui.set_gap(container, 8.0);

        // Color preview
        let preview = ui.create_widget(container, WidgetType::Panel);
        ui.set_size(preview, Size::fixed(50.0, 50.0));
        ui.set_background(preview, Color::from_array(self.color));
        ui.set_corner_radius(preview, 8.0);
        ui.set_border(preview, Border::all(1.0, Colors::border_default()));

        // RGB sliders
        for (i, (name, channel)) in [
            ("R", self.color[0]),
            ("G", self.color[1]),
            ("B", self.color[2]),
        ].iter().enumerate() {
            let row = ui.create_widget(container, WidgetType::Panel);
            ui.set_layout(row, Layout::horizontal());
            ui.set_gap(row, 8.0);

            ui.create_widget(row, WidgetType::Label {
                text: name.to_string(),
                size: FontSize::Sm,
                weight: FontWeight::Medium,
            });

            let slider = ui.create_widget(row, WidgetType::Slider {
                value: *channel,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            });
            ui.set_size(slider, Size::fill_width());

            let idx = i;
            ui.on_change(slider, move |value| {
                self.color[idx] = value;
            });
        }

        // Alpha slider (optional)
        if self.show_alpha {
            let row = ui.create_widget(container, WidgetType::Panel);
            ui.set_layout(row, Layout::horizontal());
            ui.set_gap(row, 8.0);

            ui.create_widget(row, WidgetType::Label {
                text: "A".into(),
                size: FontSize::Sm,
                weight: FontWeight::Medium,
            });

            let slider = ui.create_widget(row, WidgetType::Slider {
                value: self.color[3],
                min: 0.0,
                max: 1.0,
                step: 0.01,
            });
            ui.set_size(slider, Size::fill_width());

            ui.on_change(slider, |value| {
                self.color[3] = value;
            });
        }

        container
    }
}

/// Gradient editor widget
pub struct GradientEditorWidget {
    pub stops: Vec<GradientStop>,
    pub selected_stop: Option<usize>,
}

pub struct GradientStop {
    pub position: f32, // 0.0 - 1.0
    pub color: [f32; 4],
}

impl GradientEditorWidget {
    pub fn build(&mut self, ui: &mut UiTree, parent: WidgetId) -> WidgetId {
        let container = ui.create_widget(parent, WidgetType::Panel);
        ui.set_layout(container, Layout::vertical());
        ui.set_gap(container, 12.0);

        // Gradient preview bar
        let preview = ui.create_widget(container, WidgetType::GradientPreview {
            stops: self.stops.clone(),
        });
        ui.set_size(preview, Size::new(Size::fill(), Size::fixed(30.0)));
        ui.set_corner_radius(preview, 4.0);

        // Stop markers - clickable
        let markers = ui.create_widget(container, WidgetType::Panel);
        ui.set_layout(markers, Layout::stack());
        ui.set_size(markers, Size::new(Size::fill(), Size::fixed(20.0)));

        for (i, stop) in self.stops.iter().enumerate() {
            let marker = ui.create_widget(markers, WidgetType::Panel);
            ui.set_size(marker, Size::fixed(12.0, 12.0));
            ui.set_background(marker, Color::from_array(stop.color));
            ui.set_border(marker, Border::all(2.0, if Some(i) == self.selected_stop {
                Colors::accent_emphasis()
            } else {
                Colors::border_default()
            }));
            ui.set_corner_radius(marker, 6.0);
            
            // Position marker
            ui.set_position(marker, Position::relative(stop.position, 0.0));

            let idx = i;
            ui.on_click(marker, move |_| {
                self.selected_stop = Some(idx);
            });

            ui.set_draggable(marker, true);
            ui.on_drag(marker, move |delta, size| {
                self.stops[idx].position = (self.stops[idx].position + delta.x / size.x)
                    .clamp(0.0, 1.0);
            });
        }

        // Selected stop editor
        if let Some(idx) = self.selected_stop {
            let stop = &mut self.stops[idx];
            
            let editor = ui.create_widget(container, WidgetType::Panel);
            ui.set_layout(editor, Layout::vertical());
            ui.set_gap(editor, 8.0);

            // Position
            let pos_row = ui.create_widget(editor, WidgetType::Panel);
            ui.set_layout(pos_row, Layout::horizontal());

            ui.create_widget(pos_row, WidgetType::Label {
                text: "Position".into(),
                size: FontSize::Sm,
                weight: FontWeight::Normal,
            });

            let slider = ui.create_widget(pos_row, WidgetType::Slider {
                value: stop.position,
                min: 0.0,
                max: 1.0,
                step: 0.01,
            });

            ui.on_change(slider, |value| {
                stop.position = value;
            });

            // Color picker
            let mut picker = ColorPickerWidget {
                color: stop.color,
                show_alpha: true,
            };
            picker.build(ui, editor);
        }

        container
    }
}
```

---

## Custom Viewport Tools

Add custom tools to the viewport:

```rust
pub struct MeasureTool {
    start: Option<Vec3>,
    end: Option<Vec3>,
    measuring: bool,
}

impl ViewportTool for MeasureTool {
    fn name(&self) -> &str {
        "Measure"
    }

    fn icon(&self) -> &str {
        "ruler"
    }

    fn shortcut(&self) -> Option<Shortcut> {
        Some(Shortcut::new(KeyCode::M, Modifiers::NONE))
    }

    fn on_activate(&mut self, viewport: &ViewportWidget) {
        self.start = None;
        self.end = None;
        self.measuring = false;
    }

    fn on_click(&mut self, viewport: &ViewportWidget, pos: Vec2, button: MouseButton) {
        if button == MouseButton::Left {
            let (origin, direction) = viewport.screen_to_ray(pos);
            
            // Raycast to get world position
            if let Some(hit) = raycast(origin, direction) {
                if self.start.is_none() {
                    self.start = Some(hit.position);
                    self.measuring = true;
                } else {
                    self.end = Some(hit.position);
                    self.measuring = false;
                }
            }
        }
    }

    fn on_mouse_move(&mut self, viewport: &ViewportWidget, pos: Vec2) {
        if self.measuring {
            let (origin, direction) = viewport.screen_to_ray(pos);
            if let Some(hit) = raycast(origin, direction) {
                self.end = Some(hit.position);
            }
        }
    }

    fn draw_overlay(&self, viewport: &ViewportWidget, gizmos: &mut GizmoRenderer) {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            // Draw line
            gizmos.line(start, end, Color::YELLOW, 2.0);

            // Draw endpoints
            gizmos.sphere(start, 0.1, Color::GREEN);
            gizmos.sphere(end, 0.1, Color::RED);

            // Draw distance label
            let distance = (end - start).length();
            let mid = (start + end) / 2.0;
            gizmos.text(mid, &format!("{:.2}m", distance), Color::WHITE);
        }
    }
}
```

---

## Registering Your Plugin

### Method 1: Built into Project

```rust
// In your game's main.rs or lib.rs
use lunaris_editor::EditorContext;

fn main() {
    let mut editor = EditorContext::new();
    
    // Register plugins
    editor.register_plugin(MyPlugin);
    editor.register_plugin(AnotherPlugin);
    
    editor.run();
}
```

### Method 2: Dynamic Plugin (DLL)

```rust
// Build as cdylib
// Cargo.toml:
// [lib]
// crate-type = ["cdylib"]

#[no_mangle]
pub fn create_plugin() -> Box<dyn EditorPlugin> {
    Box::new(MyPlugin)
}
```

Place the `.dll` / `.so` in `plugins/` folder.

---

## Plugin Manifest

Create `plugin.toml`:

```toml
[plugin]
name = "My Plugin"
version = "1.0.0"
author = "Your Name"
description = "Adds cool features to the editor"

[dependencies]
lunaris-editor = "0.1"

[panels]
my_panel = { title = "My Panel", icon = "custom_icon", default_dock = "right" }

[inspectors]
my_component = "MyComponent"

[tools]
measure = { name = "Measure Tool", icon = "ruler", shortcut = "M" }
```

---

## Best Practices

### 1. Use Theme Colors
```rust
// ✅ Good
ui.set_background(widget, Colors::bg_subtle());

// ❌ Bad
ui.set_background(widget, Color::hex("#1a1a1a"));
```

### 2. Consistent Spacing
```rust
// Use the spacing scale
ui.set_padding(widget, Spacing::all(tokens.spacing.s4)); // 16px
ui.set_gap(widget, tokens.spacing.s2); // 8px
```

### 3. Handle Updates Efficiently
```rust
fn update(&mut self, ctx: &EditorContext, dt: f32) {
    // Only update when necessary
    if self.needs_refresh {
        self.refresh_data(ctx);
        self.needs_refresh = false;
    }
}
```

### 4. Localization Ready
```rust
ui.create_widget(parent, WidgetType::Label {
    text: t!("my_plugin.title"), // Use translation key
    ..
});
```

---

## Example: Complete Asset Browser Plugin

```rust
pub struct TexturePreviewPlugin;

impl EditorPlugin for TexturePreviewPlugin {
    fn name(&self) -> &str { "Texture Preview" }
    fn version(&self) -> &str { "1.0.0" }

    fn register(&self, editor: &mut EditorContext) {
        editor.register_panel::<TexturePreviewPanel>();
    }
}

pub struct TexturePreviewPanel {
    current_texture: Option<Handle<Texture>>,
    zoom: f32,
    show_alpha: bool,
    show_channels: ChannelView,
}

enum ChannelView { All, Red, Green, Blue, Alpha }

impl EditorPanel for TexturePreviewPanel {
    fn id(&self) -> &str { "texture_preview" }
    fn title(&self) -> &str { "Texture Preview" }
    fn icon(&self) -> &str { "image" }

    fn build(&mut self, ui: &mut UiTree, ctx: &EditorContext) -> WidgetId {
        let panel = ui.create_widget(ui.root(), WidgetType::Panel);
        ui.set_layout(panel, Layout::vertical());

        // Toolbar
        let toolbar = ui.create_widget(panel, WidgetType::Panel);
        ui.set_layout(toolbar, Layout::horizontal());
        ui.set_padding(toolbar, Spacing::all(8.0));
        ui.set_gap(toolbar, 8.0);

        // Zoom controls
        for (label, zoom) in [("25%", 0.25), ("50%", 0.5), ("100%", 1.0), ("200%", 2.0)] {
            let btn = ui.create_widget(toolbar, WidgetType::Button {
                label: label.into(),
                icon: None,
                variant: if (self.zoom - zoom).abs() < 0.01 {
                    ButtonVariant::Primary
                } else {
                    ButtonVariant::Ghost
                },
            });

            let z = zoom;
            ui.on_click(btn, move |_| {
                self.zoom = z;
            });
        }

        // Channel selector
        let channel_dropdown = ui.create_widget(toolbar, WidgetType::Dropdown {
            options: vec!["RGB", "R", "G", "B", "A"],
            selected: match self.show_channels {
                ChannelView::All => 0,
                ChannelView::Red => 1,
                ChannelView::Green => 2,
                ChannelView::Blue => 3,
                ChannelView::Alpha => 4,
            },
        });

        ui.on_change(channel_dropdown, |idx| {
            self.show_channels = match idx {
                1 => ChannelView::Red,
                2 => ChannelView::Green,
                3 => ChannelView::Blue,
                4 => ChannelView::Alpha,
                _ => ChannelView::All,
            };
        });

        // Image preview
        if let Some(texture) = &self.current_texture {
            let preview = ui.create_widget(panel, WidgetType::Image {
                texture: texture.clone(),
                fit: ImageFit::Contain,
            });
            ui.set_size(preview, Size::fill_both());
            ui.set_transform(preview, Transform::scale(self.zoom, self.zoom));
        } else {
            let placeholder = ui.create_widget(panel, WidgetType::Label {
                text: "Select a texture to preview".into(),
                size: FontSize::Lg,
                weight: FontWeight::Normal,
            });
            ui.set_text_color(placeholder, Colors::fg_muted());
        }

        panel
    }

    fn on_selection_changed(&mut self, ctx: &EditorContext) {
        // Update preview when selection changes
        if let Some(asset) = ctx.selection.selected_asset() {
            if asset.is::<Texture>() {
                self.current_texture = Some(asset.handle());
            }
        }
    }
}
```

---

## 🚀 Publish Your Plugin

1. **Create repository** on GitHub
2. **Add to registry**: Submit to `plugins.lunaris.dev`
3. **Users install**: `lunaris plugin install your-plugin`

---

**Happy plugin development! 🔌**

The community can't wait to see what you build.
