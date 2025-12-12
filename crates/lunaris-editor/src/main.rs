//! Lunaris Editor Entry Point

use lunaris_editor::{Editor, EditorConfig};
use lunaris_runtime::{Application, AppRunner, Window, WindowConfig};
use lunaris_renderer::gpu::{GraphicsContext, GraphicsConfig};
use lunaris_core::{input::Input, math::Color, Result};
use std::sync::Arc;

struct EditorApp {
    editor: Editor,
    graphics: Option<GraphicsContext>,
}

impl Application for EditorApp {
    fn init(&mut self, window: &Window) {
        tracing::info!("Initializing editor graphics...");

        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        let config = GraphicsConfig::default();
        
        // Initialize graphics
        let mut graphics = rt.block_on(async {
            GraphicsContext::new(&config).await
        }).expect("Failed to initialize graphics");
        
        graphics.set_clear_color(Color::new(0.1, 0.1, 0.1, 1.0)); // Dark grey

        // Create surface
        if let Some(raw_window) = window.raw() {
            let size = window.size();
            graphics.create_surface(raw_window.clone(), size.0, size.1)
                .expect("Failed to create surface");
        }

        self.graphics = Some(graphics);
        tracing::info!("Editor graphics initialized");
    }

    fn update(&mut self, _input: &Input, _delta_time: f32) {
        // self.editor.update(input, (1600, 900)); // TODO: Pass real size
    }

    fn render(&mut self, _window: &Window) {
        if let Some(graphics) = &self.graphics {
            if let Some(frame) = graphics.begin_frame() {
                let mut encoder = graphics.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Editor Render Encoder"),
                });

                {
                    let _pass = graphics.create_render_pass(&mut encoder, &frame.view);
                }

                graphics.queue().submit(Some(encoder.finish()));
                graphics.end_frame(frame);
            }
        }
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        if let Some(graphics) = &mut self.graphics {
            graphics.resize(width, height);
        }
    }
}

fn main() -> Result<()> {
    lunaris_runtime::init()?;

    let config = WindowConfig::new("Lunaris Editor")
        .with_size(1600, 900)
        .with_resizable(true);

    let app = EditorApp {
        editor: Editor::new(),
        graphics: None,
    };

    AppRunner::new(app, config).run()
}
