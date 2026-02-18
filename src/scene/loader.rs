use bevy::prelude::*;
use crate::script::runner::{Instruction, ScriptRunner};

pub fn load_test_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut runner: ResMut<ScriptRunner>,
) {
    // Spawn camera (new Bevy style)
    commands.spawn(Camera2d);

    // Load texture
    let texture: Handle<Image> = asset_server.load("images/bg.png");

    // Spawn sprite (new Bevy style)
    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Test script
    runner.instructions = vec![
        Instruction::Say("Welcome to the VN engine prototype.".to_string()),
        Instruction::Say("Press SPACE to continue dialogue.".to_string()),
        Instruction::Say("Script runner works.".to_string()),
    ];
}
