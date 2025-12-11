# Example Games

This directory contains example games demonstrating how to develop games with the Lunaris Engine.

## Pong (Rust)

A simple Pong implementation using pure Rust:

```rust
use lunaris_core::{Game, GameConfig, Input, Time};

struct MyGame;

impl Game for MyGame {
    fn new() -> Result<Self> { Ok(Self) }
    fn update(&mut self, time: &Time, input: &Input) { /* game logic */ }
    fn render(&mut self) { /* drawing */ }
}

lunaris_main!(MyGame);
```

## Pong (Lua)

The same Pong game implemented in Lua scripting:

```lua
function on_init()
    print("Game started!")
end

function on_update(dt)
    if lunaris.input.is_key_down("w") then
        -- move paddle
    end
end

function on_render()
    -- draw game
end
```

## Running Examples

```bash
# Rust example (requires building)
cargo run --example pong_rust

# Lua example (run through engine)
lunaris run examples/pong_lua/main.lua
```

## Creating Your Own Game

### Option 1: Pure Rust (Maximum Performance)

Best for:
- Engine-level performance requirements
- Direct hardware access
- Complex systems (physics, rendering)

### Option 2: Lua Scripting (Rapid Development)

Best for:
- Gameplay logic
- Prototyping
- Modding support
- Hot-reloading during development

### Option 3: Hybrid (Recommended)

Use Rust for core systems and Lua for gameplay:

```rust
// Rust: Define game systems
struct Physics { /* ... */ }
struct Renderer { /* ... */ }

// Expose to Lua
engine.register("physics", physics);
engine.register("renderer", renderer);
```

```lua
-- Lua: Gameplay logic
function on_update(dt)
    local hit = lunaris.physics.raycast(from, to)
    if hit then
        lunaris.audio.play("explosion")
    end
end
```
