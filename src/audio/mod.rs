use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MusicManager {
    pub current: Option<Entity>,
}

#[derive(Component)]
pub struct MusicTag;

pub fn play_music(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    manager: &mut ResMut<MusicManager>,
    path: String,
) {
    // Stop current music
    if let Some(entity) = manager.current.take() {
        commands.entity(entity).despawn();
    }

    let source: Handle<AudioSource> =
        asset_server.load(format!("audio/music/{}", path));

    let entity = commands.spawn((
        AudioPlayer::new(source),
        PlaybackSettings::LOOP,
        MusicTag,
    )).id();

    manager.current = Some(entity);
}

pub fn stop_music(
    commands: &mut Commands,
    manager: &mut ResMut<MusicManager>,
) {
    if let Some(entity) = manager.current.take() {
        commands.entity(entity).despawn();
    }
}

pub fn play_sfx(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    path: String,
) {
    let source: Handle<AudioSource> =
        asset_server.load(format!("audio/sfx/{}", path));

    commands.spawn((
        AudioPlayer::new(source),
        PlaybackSettings::DESPAWN,
    ));
}
