use bevy::prelude::*;
use crate::script::runner::{Instruction, ScriptRunner};

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

    runner.instructions = vec![
        Instruction::Label("start".into()),
        Instruction::Say("Where do you want to go?".into()),
        Instruction::Choice(vec![
            ("Go left".into(), "left_path".into()),
            ("Go right".into(), "right_path".into()),
        ]),

        Instruction::Label("left_path".into()),
        Instruction::Say("You went left.".into()),
        Instruction::JumpLabel("end".into()),

        Instruction::Label("right_path".into()),
        Instruction::Say("You went right.".into()),

        Instruction::Label("end".into()),
        Instruction::Say("End of demo.".into()),
    ];

    runner.rebuild_labels();
}
