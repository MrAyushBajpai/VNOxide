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
}

impl Default for TransformParams {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            scale: None,
            rotation_deg: None,
            preset: None,
        }
    }
}

fn preset_position(name: &str) -> Vec3 {
    match name {
        "left" => Vec3::new(-400.0, -100.0, 10.0),
        "center" => Vec3::new(0.0, -100.0, 10.0),
        "right" => Vec3::new(400.0, -100.0, 10.0),
        _ => Vec3::new(0.0, -100.0, 10.0),
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

    // Start from preset or default
    let mut translation = if let Some(ref preset) = params.preset {
        preset_position(preset)
    } else {
        Vec3::new(0.0, -100.0, 10.0)
    };

    // Override x/y if provided
    if let Some(x) = params.x {
        translation.x = x;
    }

    if let Some(y) = params.y {
        translation.y = y;
    }

    let mut transform = Transform::from_translation(translation);

    // Scale
    if let Some(scale) = params.scale {
        transform.scale = Vec3::splat(scale);
    }

    // Rotation
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
