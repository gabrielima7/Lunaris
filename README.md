<p align="center">
  <img src="docs/assets/logo.svg" alt="Lunaris Engine" width="400">
</p>

<h1 align="center">🌙 Lunaris Engine</h1>

<p align="center">
  <strong>The Rust Game Engine That Changes Everything</strong>
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#-why-lunaris">Why Lunaris</a> •
  <a href="#-docs">Docs</a> •
  <a href="#-community">Community</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Lines%20of%20Code-51K+-blue?style=for-the-badge" alt="Lines of Code">
  <img src="https://img.shields.io/badge/Royalties-0%25-green?style=for-the-badge" alt="Royalties">
  <img src="https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge" alt="Made with Rust">
</p>

---

## ⚡ Quick Start

```bash
# Install
cargo install lunaris-cli

# Create project
lunaris new my_game
cd my_game

# Run editor
lunaris editor
```

**That's it!** You're ready to build your game.

---

## 🎮 What Can You Build?

<table>
<tr>
<td width="33%">

### 🏃 Platformers
Tight controls, pixel-perfect physics, beautiful 2D.

</td>
<td width="33%">

### 🗡️ Action RPGs
Combat system, inventory, quests, dialogue.

</td>
<td width="33%">

### 🌍 Open Worlds
Procedural terrain, day/night, weather.

</td>
</tr>
<tr>
<td width="33%">

### 🔫 Shooters
FPS, TPS, multiplayer with prediction.

</td>
<td width="33%">

### 🎲 Strategy
RTS, turn-based, AI behaviors.

</td>
<td width="33%">

### 🎭 VR/AR
Quest, Vision Pro, all headsets.

</td>
</tr>
</table>

---

## ✨ Features

### 🖼️ Graphics
- **Lumen GI** - Real-time global illumination
- **Nanite** - Millions of polygons, no LODs
- **Ray Tracing** - Beautiful reflections & shadows
- **MetaHuman** - Photorealistic characters

### 🎯 Gameplay
- **Visual Scripting** - No code required
- **Lua Scripting** - For rapid prototyping
- **Rust** - When you need performance
- **AI Copilot** - Generate code with AI

### 🎨 Editor
- **Modern UI** - Clean, fast, customizable
- **Live Reload** - See changes instantly
- **One-Click Build** - PC, Console, Mobile, VR

### 🌐 Multiplayer
- **Built-in** - Replication, RPCs, prediction
- **Any Scale** - 2 players to MMO

---

## 💡 Why Lunaris?

| | Unity | Unreal | Lunaris |
|---|:---:|:---:|:---:|
| **Price** | $2K+/year | 5% royalty | **Free forever** |
| **Performance** | Good | Great | **Best** (Rust) |
| **Safety** | Runtime errors | C++ crashes | **Compile-time safe** |
| **Open Source** | ❌ | Partial | **✅ Full** |
| **2D Support** | Plugin | Poor | **Native** |
| **Learning Curve** | Medium | Hard | **Easy** |

---

## 📝 Code Example

**Rust (for performance):**
```rust
#[derive(Component)]
struct Player { speed: f32 }

fn movement(input: Res<Input>, mut q: Query<(&Player, &mut Transform)>) {
    for (player, mut tf) in q.iter_mut() {
        tf.position += input.movement() * player.speed;
    }
}
```

**Lua (for prototyping):**
```lua
function on_update(entity, dt)
    local speed = 5.0
    entity.position = entity.position + input.movement * speed * dt
end
```

---

## 📚 Learn

| Resource | Description |
|----------|-------------|
| [📖 Getting Started](docs/getting_started.md) | Your first project |
| [🎮 Examples](examples/) | Working games to learn from |
| [🔄 Unity Migration](docs/tutorials/unity_migration.md) | Coming from Unity? |
| [📺 YouTube](https://youtube.com/@lunaris) | Video tutorials |

---

## 🌟 Showcase

### Vertical Slice Demo
*A complete game showcasing all engine features*

### AAA Rendering Demo  
*50+ million triangles with Lumen and Nanite*

### Action RPG Demo
*Combat, inventory, quests, dialogue*

---

## 🤝 Community

<p align="center">
  <a href="https://discord.gg/lunaris"><img src="https://img.shields.io/badge/Discord-Join%20Us-5865F2?style=for-the-badge&logo=discord" alt="Discord"></a>
  <a href="https://twitter.com/lunarisengine"><img src="https://img.shields.io/badge/Twitter-Follow-1DA1F2?style=for-the-badge&logo=twitter" alt="Twitter"></a>
  <a href="https://github.com/lunaris/engine"><img src="https://img.shields.io/badge/GitHub-Star-181717?style=for-the-badge&logo=github" alt="GitHub"></a>
</p>

---

## 📊 Stats

<p align="center">
  <img src="https://img.shields.io/badge/Rust%20Files-132-informational?style=flat-square" alt="Rust Files">
  <img src="https://img.shields.io/badge/Lines-51K+-informational?style=flat-square" alt="Lines">
  <img src="https://img.shields.io/badge/Editor%20Modules-20-informational?style=flat-square" alt="Modules">
  <img src="https://img.shields.io/badge/Platforms-10+-informational?style=flat-square" alt="Platforms">
</p>

---

## 🛠️ Built With

- **Rust** - Safe, fast, concurrent
- **wgpu** - Modern graphics API
- **glam** - Fast math library
- **mlua** - Lua scripting
- **serde** - Serialization

---

## 📄 License

MIT License - **Use it for anything, forever.**

No royalties. No subscriptions. No restrictions.

---

<p align="center">
  <strong>Ready to build your dream game?</strong>
</p>

<p align="center">
  <code>cargo install lunaris-cli && lunaris new my_game</code>
</p>

<p align="center">
  Made with 🌙 by game developers, for game developers.
</p>