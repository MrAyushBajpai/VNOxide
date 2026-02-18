use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct BackgroundManager {
    pub current: Option<Entity>,
}

#[derive(Component)]
pub struct BackgroundTag;

pub fn set_background_image(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    manager: &mut ResMut<BackgroundManager>,
    path: String,
) {
    // Remove existing background
    if let Some(entity) = manager.current.take() {
        commands.entity(entity).despawn();
    }

    let texture: Handle<Image> = asset_server.load(format!("backgrounds/{}", path));

    let entity = commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0.0, 0.0, 0.0), // layer 0
        BackgroundTag,
    )).id();

    manager.current = Some(entity);
}

pub fn set_background_color(
    commands: &mut Commands,
    manager: &mut ResMut<BackgroundManager>,
    color: Color,
) {
    // Remove existing background
    if let Some(entity) = manager.current.take() {
        commands.entity(entity).despawn();
    }

    // Large quad to simulate a background color
    let entity = commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(5000.0, 5000.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        BackgroundTag,
    )).id();

    manager.current = Some(entity);
}
