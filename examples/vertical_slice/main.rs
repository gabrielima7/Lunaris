//! Vertical Slice Demo - Main Entry Point
//!
//! Run with: cargo run --example vertical_slice

mod game;

pub use game::*;

fn main() {
    println!("=================================");
    println!("  LUNARIS ENGINE - VERTICAL SLICE");
    println!("=================================");
    println!();
    
    // Initialize demo
    let mut demo = VerticalSlice::new(DemoConfig::default());
    
    println!("Demo initialized!");
    println!("Config: {:?}", demo.config);
    println!();
    
    // Start game
    demo.start();
    println!("Game started!");
    println!();
    
    // Simulate a few frames
    for frame in 0..60 {
        let dt = 1.0 / 60.0;
        demo.update(dt);
        
        // Simulate player attack every 10 frames
        if frame % 10 == 0 {
            demo.player_attack();
        }
        
        // Print stats every 20 frames
        if frame % 20 == 0 {
            let stats = demo.stats();
            println!("Frame {}: Score={}, Kills={}, Enemies={}", 
                frame, stats.score, stats.kills, stats.enemies_remaining);
        }
    }
    
    println!();
    println!("Final Stats:");
    let final_stats = demo.stats();
    println!("  Game Time: {:.2}s", final_stats.game_time);
    println!("  Player Level: {}", final_stats.player_level);
    println!("  Player Health: {:.0}", final_stats.player_health);
    println!("  Kills: {}", final_stats.kills);
    println!("  Score: {}", final_stats.score);
    println!("  Enemies Remaining: {}", final_stats.enemies_remaining);
    println!("  Quests Completed: {}", final_stats.quests_completed);
    println!();
    println!("Demo complete!");
}
