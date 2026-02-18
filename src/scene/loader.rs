use bevy::prelude::*;
use crate::script::runner::ScriptRunner;
use crate::script::loader::load_script;


pub fn load_test_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut runner: ResMut<ScriptRunner>,
) {
    commands.spawn(Camera2d);

    let texture: Handle<Image> = asset_server.load("images/bg.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Load script from file
    runner.instructions = load_script("scripts/test.vn");
    runner.rebuild_labels();
    runner.ip = 0;
}
