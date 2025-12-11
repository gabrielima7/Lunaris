//! Extended Lua API for game development
//!
//! Provides game-specific functions to Lua scripts.

use crate::error::ScriptResult;
use crate::sandbox::ScriptEngine;
use mlua::{Function, Lua, Table, Value};

/// Register game APIs with the Lua environment
///
/// # Errors
///
/// Returns an error if API registration fails
pub fn register_game_api(engine: &ScriptEngine, lua: &Lua) -> ScriptResult<()> {
    let globals = lua.globals();
    let lunaris: Table = globals.get("lunaris")?;

    // Input API
    let input = lua.create_table()?;
    register_input_api(lua, &input)?;
    lunaris.set("input", input)?;

    // Entity API (placeholder for ECS integration)
    let entity = lua.create_table()?;
    register_entity_api(lua, &entity)?;
    lunaris.set("entity", entity)?;

    // Audio API
    let audio = lua.create_table()?;
    register_audio_api(lua, &audio)?;
    lunaris.set("audio", audio)?;

    // Physics API
    let physics = lua.create_table()?;
    register_physics_api(lua, &physics)?;
    lunaris.set("physics", physics)?;

    // Scene API
    let scene = lua.create_table()?;
    register_scene_api(lua, &scene)?;
    lunaris.set("scene", scene)?;

    Ok(())
}

fn register_input_api(lua: &Lua, table: &Table) -> ScriptResult<()> {
    // Note: These would be connected to actual input state in real implementation
    
    table.set(
        "is_key_down",
        lua.create_function(|_, key: String| {
            // Placeholder - would check actual input state
            tracing::debug!(target: "lua", "is_key_down: {}", key);
            Ok(false)
        })?,
    )?;

    table.set(
        "is_key_pressed",
        lua.create_function(|_, key: String| {
            tracing::debug!(target: "lua", "is_key_pressed: {}", key);
            Ok(false)
        })?,
    )?;

    table.set(
        "is_mouse_down",
        lua.create_function(|_, button: u32| {
            tracing::debug!(target: "lua", "is_mouse_down: {}", button);
            Ok(false)
        })?,
    )?;

    table.set(
        "get_mouse_position",
        lua.create_function(|lua, ()| {
            let result = lua.create_table()?;
            result.set("x", 0.0_f64)?;
            result.set("y", 0.0_f64)?;
            Ok(result)
        })?,
    )?;

    table.set(
        "get_axis",
        lua.create_function(|_, axis: String| {
            tracing::debug!(target: "lua", "get_axis: {}", axis);
            Ok(0.0_f64)
        })?,
    )?;

    Ok(())
}

fn register_entity_api(lua: &Lua, table: &Table) -> ScriptResult<()> {
    // Entity creation
    table.set(
        "create",
        lua.create_function(|lua, name: Option<String>| {
            let entity = lua.create_table()?;
            entity.set("id", lunaris_core::id::Id::new().raw())?;
            entity.set("name", name.unwrap_or_else(|| "Entity".to_string()))?;
            
            // Default transform
            let transform = lua.create_table()?;
            transform.set("x", 0.0_f64)?;
            transform.set("y", 0.0_f64)?;
            transform.set("rotation", 0.0_f64)?;
            transform.set("scale_x", 1.0_f64)?;
            transform.set("scale_y", 1.0_f64)?;
            entity.set("transform", transform)?;
            
            tracing::debug!(target: "lua", "Created entity");
            Ok(entity)
        })?,
    )?;

    // Get entity position
    table.set(
        "get_position",
        lua.create_function(|lua, entity: Table| {
            let transform: Table = entity.get("transform")?;
            let result = lua.create_table()?;
            result.set("x", transform.get::<_, f64>("x")?)?;
            result.set("y", transform.get::<_, f64>("y")?)?;
            Ok(result)
        })?,
    )?;

    // Set entity position
    table.set(
        "set_position",
        lua.create_function(|_, (entity, pos): (Table, Table)| {
            let transform: Table = entity.get("transform")?;
            transform.set("x", pos.get::<_, f64>("x")?)?;
            transform.set("y", pos.get::<_, f64>("y")?)?;
            Ok(())
        })?,
    )?;

    // Move entity
    table.set(
        "move",
        lua.create_function(|_, (entity, dx, dy): (Table, f64, f64)| {
            let transform: Table = entity.get("transform")?;
            let x: f64 = transform.get("x")?;
            let y: f64 = transform.get("y")?;
            transform.set("x", x + dx)?;
            transform.set("y", y + dy)?;
            Ok(())
        })?,
    )?;

    // Get entity rotation
    table.set(
        "get_rotation",
        lua.create_function(|_, entity: Table| {
            let transform: Table = entity.get("transform")?;
            Ok(transform.get::<_, f64>("rotation")?)
        })?,
    )?;

    // Set entity rotation
    table.set(
        "set_rotation",
        lua.create_function(|_, (entity, rotation): (Table, f64)| {
            let transform: Table = entity.get("transform")?;
            transform.set("rotation", rotation)?;
            Ok(())
        })?,
    )?;

    Ok(())
}

fn register_audio_api(lua: &Lua, table: &Table) -> ScriptResult<()> {
    table.set(
        "play",
        lua.create_function(|_, sound: String| {
            tracing::info!(target: "lua", "Playing sound: {}", sound);
            Ok(())
        })?,
    )?;

    table.set(
        "stop",
        lua.create_function(|_, sound: String| {
            tracing::info!(target: "lua", "Stopping sound: {}", sound);
            Ok(())
        })?,
    )?;

    table.set(
        "set_volume",
        lua.create_function(|_, (sound, volume): (String, f64)| {
            tracing::debug!(target: "lua", "Set volume {} = {}", sound, volume);
            Ok(())
        })?,
    )?;

    Ok(())
}

fn register_physics_api(lua: &Lua, table: &Table) -> ScriptResult<()> {
    table.set(
        "raycast",
        lua.create_function(|lua, (from_x, from_y, to_x, to_y): (f64, f64, f64, f64)| {
            tracing::debug!(target: "lua", "Raycast from ({}, {}) to ({}, {})", from_x, from_y, to_x, to_y);
            
            // Return nil for no hit, or a table with hit info
            let result = lua.create_table()?;
            result.set("hit", false)?;
            Ok(result)
        })?,
    )?;

    table.set(
        "check_collision",
        lua.create_function(|_, (entity_a, entity_b): (Table, Table)| {
            let _id_a: u64 = entity_a.get("id")?;
            let _id_b: u64 = entity_b.get("id")?;
            // Placeholder collision check
            Ok(false)
        })?,
    )?;

    Ok(())
}

fn register_scene_api(lua: &Lua, table: &Table) -> ScriptResult<()> {
    table.set(
        "load",
        lua.create_function(|_, scene_name: String| {
            tracing::info!(target: "lua", "Loading scene: {}", scene_name);
            Ok(())
        })?,
    )?;

    table.set(
        "get_current",
        lua.create_function(|_, ()| {
            Ok("main")
        })?,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SandboxConfig, ScriptEngine};

    #[test]
    fn entity_api_works() {
        let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();
        
        let result: u64 = engine
            .eval(
                r#"
                local player = lunaris.entity.create("Player")
                lunaris.entity.move(player, 10, 20)
                local pos = lunaris.entity.get_position(player)
                return pos.x
            "#,
            )
            .unwrap();
        
        assert_eq!(result, 10);
    }
}
