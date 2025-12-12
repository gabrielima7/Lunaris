//! Hello World Example
//!
//! The simplest possible Lunaris game.
//!
//! Run with: cargo run --example hello_world

use lunaris_core::Result;

fn main() -> Result<()> {
    // Initialize Lunaris
    lunaris_core::init()?;
    
    println!("===============================");
    println!("  🌙 Hello, Lunaris Engine!");
    println!("===============================");
    println!();
    println!("Lunaris Engine v{}", lunaris_core::VERSION);
    println!();
    println!("Features:");
    println!("  ✅ Lumen-like Global Illumination");
    println!("  ✅ Nanite-like Virtualized Geometry");
    println!("  ✅ Hardware Ray Tracing");
    println!("  ✅ MetaHuman Digital Humans");
    println!("  ✅ Chaos-like Physics");
    println!("  ✅ MetaSounds Procedural Audio");
    println!("  ✅ Blueprints Visual Scripting");
    println!("  ✅ AI Copilot");
    println!("  ✅ 18+ Platform Support");
    println!();
    println!("Ready to create amazing games!");
    
    Ok(())
}
