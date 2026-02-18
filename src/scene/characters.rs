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

#[derive(Debug, Clone)]
pub struct TransformParams {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub scale: Option<f32>,
    pub rotation_deg: Option<f32>,
    pub preset: Option<String>,
    pub layer: Option<f32>,
}

impl Default for TransformParams {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            scale: None,
            rotation_deg: None,
            preset: None,
            layer: None,
        }
    }
}

fn preset_position(name: &str) -> (f32, f32) {
    match name {
        "left" => (-400.0, -100.0),
        "center" => (0.0, -100.0),
        "right" => (400.0, -100.0),
        _ => (0.0, -100.0),
    }
}

pub fn show_character(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    manager: &mut ResMut<CharacterManager>,
    name: String,
    expression: String,
    params: TransformParams,
) {
    // Remove existing sprite
    if let Some(entity) = manager.active.remove(&name) {
        commands.entity(entity).despawn();
    }

    let path = format!("characters/{}/{}.png", name, expression);
    let texture: Handle<Image> = asset_server.load(path);

    // Base position
    let (mut x, mut y) = if let Some(ref preset) = params.preset {
        preset_position(preset)
    } else {
        (0.0, -100.0)
    };

    // Overrides
    if let Some(v) = params.x { x = v; }
    if let Some(v) = params.y { y = v; }

    // Layer (z)
    let z = params.layer.unwrap_or(10.0);

    let mut transform = Transform::from_xyz(x, y, z);

    if let Some(scale) = params.scale {
        transform.scale = Vec3::splat(scale);
    }

    if let Some(rot) = params.rotation_deg {
        transform.rotation = Quat::from_rotation_z(rot.to_radians());
    }

    let entity = commands.spawn((
        Sprite::from_image(texture),
        transform,
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
