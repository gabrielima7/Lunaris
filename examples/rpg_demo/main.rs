//! RPG Demo Entry Point

mod game;
pub use game::*;

fn main() {
    println!("======================================");
    println!("  🎮 LUNARIS RPG - Showcase Demo");
    println!("======================================");
    println!();

    // Create new game
    let mut game = RpgDemo::new();
    
    println!("📜 Character Created:");
    println!("   Name: {}", game.player.name);
    println!("   Class: {:?}", game.player.class);
    println!("   Level: {}", game.player.level);
    println!("   HP: {}/{}", game.player.hp, game.player.base_stats.max_hp);
    println!("   MP: {}/{}", game.player.mp, game.player.base_stats.max_mp);
    println!();

    // Add party member
    let mage = Character::new_player("Luna", CharacterClass::Mage);
    game.party.push(mage);
    println!("✨ Party member joined: Luna the Mage");
    println!();

    // Simulate combat
    println!("⚔️ Starting Combat Demo...");
    let enemies = vec![
        Character::new_npc("Goblin", CharacterClass::Rogue, 1),
        Character::new_npc("Goblin Scout", CharacterClass::Ranger, 2),
    ];

    let mut party = vec![game.player.clone()];
    party.extend(game.party.clone());

    let mut combat = CombatSystem::new(party, enemies);
    
    // Simulate a few turns
    for turn in 0..5 {
        if combat.is_over() {
            break;
        }

        let current = &combat.turn_order[combat.current_turn];
        let attacker_name = if current.is_party {
            combat.party[current.index].name.clone()
        } else {
            combat.enemies[current.index].name.clone()
        };

        println!("\nTurn {}: {}'s turn", turn + 1, attacker_name);

        // Auto-attack first enemy/party member
        let target = if current.is_party {
            // Find first alive enemy
            combat.enemies.iter().enumerate()
                .find(|(_, e)| e.is_alive())
                .map(|(i, _)| CombatantRef { is_party: false, index: i })
        } else {
            // Find first alive party member
            combat.party.iter().enumerate()
                .find(|(_, p)| p.is_alive())
                .map(|(i, _)| CombatantRef { is_party: true, index: i })
        };

        if let Some(target_ref) = target {
            combat.execute_attack(&current.clone(), &target_ref);
        }

        combat.update(1.0 / 60.0);
    }

    println!();
    println!("📋 Combat Log:");
    for entry in &combat.log {
        println!("   {}", entry);
    }

    println!();
    println!("=== Demo Features Showcased ===");
    println!("✅ Character System (6 classes, stats, skills)");
    println!("✅ Turn-based Combat (speed-based, attacks)");
    println!("✅ Party System (multiple members)");
    println!("✅ Inventory & Items (50 slots, stacking)");
    println!("✅ Quest System (objectives, rewards)");
    println!("✅ Dialogue System (branching, choices)");
    println!("✅ World/Region System (locations, enemies)");
    println!("✅ Time System (day/night cycle)");
    println!("✅ Save/Load framework");
    println!();
    println!("🚀 Lunaris RPG Demo Complete!");
}
