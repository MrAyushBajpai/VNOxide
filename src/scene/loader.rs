use bevy::prelude::*;
use crate::script::runner::{Instruction, ScriptRunner};

pub fn load_test_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut runner: ResMut<ScriptRunner>,
) {
    // Camera (new Bevy style)
    commands.spawn(Camera2d);

    // Background image
    let texture: Handle<Image> = asset_server.load("images/bg.png");

    commands.spawn((
        Sprite::from_image(texture),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Test script with labels
    runner.instructions = vec![
        Instruction::Label("start".into()),
        Instruction::Say("This is the start.".into()),
        Instruction::JumpLabel("ending".into()),

        Instruction::Label("middle".into()),
        Instruction::Say("You should NOT see this.".into()),

        Instruction::Label("ending".into()),
        Instruction::Say("This is the ending.".into()),
    ];

    // Build label lookup table
    runner.rebuild_labels();
}
