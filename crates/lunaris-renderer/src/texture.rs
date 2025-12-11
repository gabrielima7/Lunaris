//! Texture and sprite management

use lunaris_core::{id::Id, math::Rect, Result};
use std::collections::HashMap;

/// Texture handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub Id);

/// Texture metadata
#[derive(Debug, Clone)]
pub struct TextureInfo {
    /// Texture width
    pub width: u32,
    /// Texture height
    pub height: u32,
    /// Texture format
    pub format: TextureFormat,
    /// Has mipmaps
    pub mipmaps: bool,
}

/// Supported texture formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    /// RGBA 8-bit per channel
    Rgba8,
    /// RGBA 8-bit per channel, sRGB
    Rgba8Srgb,
    /// Single channel (grayscale)
    R8,
    /// Depth buffer
    Depth32,
    /// Depth + Stencil
    Depth24Stencil8,
}

/// A sprite is a rectangular region of a texture
#[derive(Debug, Clone)]
pub struct Sprite {
    /// Source texture
    pub texture: TextureId,
    /// UV coordinates (normalized 0-1)
    pub uv_rect: Rect,
    /// Pixel size
    pub size: (u32, u32),
    /// Pivot point (0-1, default center)
    pub pivot: (f32, f32),
}

impl Sprite {
    /// Create a sprite from a full texture
    #[must_use]
    pub fn from_texture(texture: TextureId, width: u32, height: u32) -> Self {
        Self {
            texture,
            uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
            size: (width, height),
            pivot: (0.5, 0.5),
        }
    }

    /// Create a sprite from a region of a texture
    #[must_use]
    pub fn from_region(
        texture: TextureId,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        texture_width: u32,
        texture_height: u32,
    ) -> Self {
        let u0 = x as f32 / texture_width as f32;
        let v0 = y as f32 / texture_height as f32;
        let u1 = (x + width) as f32 / texture_width as f32;
        let v1 = (y + height) as f32 / texture_height as f32;

        Self {
            texture,
            uv_rect: Rect::new(u0, v0, u1 - u0, v1 - v0),
            size: (width, height),
            pivot: (0.5, 0.5),
        }
    }
}

/// Sprite atlas for efficient batching
#[derive(Debug)]
pub struct SpriteAtlas {
    /// The atlas texture
    pub texture: TextureId,
    /// Texture dimensions
    pub width: u32,
    pub height: u32,
    /// Named sprites within the atlas
    sprites: HashMap<String, Sprite>,
}

impl SpriteAtlas {
    /// Create a new empty atlas
    #[must_use]
    pub fn new(texture: TextureId, width: u32, height: u32) -> Self {
        Self {
            texture,
            width,
            height,
            sprites: HashMap::new(),
        }
    }

    /// Add a sprite region to the atlas
    pub fn add_sprite(&mut self, name: impl Into<String>, x: u32, y: u32, width: u32, height: u32) {
        let sprite = Sprite::from_region(self.texture, x, y, width, height, self.width, self.height);
        self.sprites.insert(name.into(), sprite);
    }

    /// Get a sprite by name
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Sprite> {
        self.sprites.get(name)
    }

    /// Create a grid-based atlas (for uniform sprite sheets)
    #[must_use]
    pub fn from_grid(
        texture: TextureId,
        texture_width: u32,
        texture_height: u32,
        sprite_width: u32,
        sprite_height: u32,
    ) -> Self {
        let mut atlas = Self::new(texture, texture_width, texture_height);

        let cols = texture_width / sprite_width;
        let rows = texture_height / sprite_height;

        for row in 0..rows {
            for col in 0..cols {
                let name = format!("sprite_{}_{}", col, row);
                atlas.add_sprite(
                    name,
                    col * sprite_width,
                    row * sprite_height,
                    sprite_width,
                    sprite_height,
                );
            }
        }

        atlas
    }
}

/// Animation frame
#[derive(Debug, Clone)]
pub struct AnimationFrame {
    /// Sprite for this frame
    pub sprite: Sprite,
    /// Duration in seconds
    pub duration: f32,
}

/// Sprite animation
#[derive(Debug, Clone)]
pub struct SpriteAnimation {
    /// Animation name
    pub name: String,
    /// Frames
    pub frames: Vec<AnimationFrame>,
    /// Whether to loop
    pub looping: bool,
}

impl SpriteAnimation {
    /// Create a new animation
    #[must_use]
    pub fn new(name: impl Into<String>, looping: bool) -> Self {
        Self {
            name: name.into(),
            frames: Vec::new(),
            looping,
        }
    }

    /// Add a frame
    pub fn add_frame(&mut self, sprite: Sprite, duration: f32) {
        self.frames.push(AnimationFrame { sprite, duration });
    }

    /// Get total duration
    #[must_use]
    pub fn total_duration(&self) -> f32 {
        self.frames.iter().map(|f| f.duration).sum()
    }

    /// Get the frame at a given time
    #[must_use]
    pub fn frame_at(&self, mut time: f32) -> Option<&AnimationFrame> {
        if self.frames.is_empty() {
            return None;
        }

        let total = self.total_duration();
        if self.looping && total > 0.0 {
            time = time % total;
        }

        let mut elapsed = 0.0;
        for frame in &self.frames {
            elapsed += frame.duration;
            if time < elapsed {
                return Some(frame);
            }
        }

        self.frames.last()
    }
}

/// Animation player component
#[derive(Debug)]
pub struct AnimationPlayer {
    /// Current animation
    pub animation: Option<SpriteAnimation>,
    /// Current time
    pub time: f32,
    /// Playback speed (1.0 = normal)
    pub speed: f32,
    /// Is playing
    pub playing: bool,
}

impl Default for AnimationPlayer {
    fn default() -> Self {
        Self {
            animation: None,
            time: 0.0,
            speed: 1.0,
            playing: false,
        }
    }
}

impl AnimationPlayer {
    /// Play an animation
    pub fn play(&mut self, animation: SpriteAnimation) {
        self.animation = Some(animation);
        self.time = 0.0;
        self.playing = true;
    }

    /// Stop the animation
    pub fn stop(&mut self) {
        self.playing = false;
        self.time = 0.0;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        self.playing = true;
    }

    /// Update the animation
    pub fn update(&mut self, delta_time: f32) {
        if self.playing {
            self.time += delta_time * self.speed;
        }
    }

    /// Get the current frame
    #[must_use]
    pub fn current_frame(&self) -> Option<&AnimationFrame> {
        self.animation.as_ref()?.frame_at(self.time)
    }

    /// Get the current sprite
    #[must_use]
    pub fn current_sprite(&self) -> Option<&Sprite> {
        self.current_frame().map(|f| &f.sprite)
    }
}
