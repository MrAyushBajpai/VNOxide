use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct CharacterManager {
    pub active: HashMap<String, Entity>,
}

#[derive(Component)]
pub struct CharacterSprite {
    pub name: String,
}

fn position_from_word(word: &str) -> f32 {
    match word {
        "left" => -400.0,
        "center" => 0.0,
        "right" => 400.0,
        _ => 0.0,
    }
}

pub fn show_character(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    manager: &mut ResMut<CharacterManager>,
    name: String,
    expression: String,
    position: String,
) {
    // Despawn existing sprite if present
    if let Some(entity) = manager.active.remove(&name) {
        commands.entity(entity).despawn();
    }

    // Expected path format:
    // assets/characters/alice/happy.png
    let path = format!("characters/{}/{}.png", name, expression);
    let texture: Handle<Image> = asset_server.load(path);

    let x = position_from_word(&position);

    let entity = commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(x, -100.0, 10.0),
        CharacterSprite { name: name.clone() },
    )).id();

    manager.active.insert(name, entity);
}

pub fn hide_character(
    commands: &mut Commands,
    manager: &mut ResMut<CharacterManager>,
    name: &str,
) {
    if let Some(entity) = manager.active.remove(name) {
        commands.entity(entity).despawn();
    }
}
