use bevy::prelude::*;
use crate::ui::dialogue::DialogueState;

#[derive(Debug, Clone)]
pub enum Instruction {
    Say(String),
    SetVar(String, i64),
    Jump(usize),
}

#[derive(Resource, Default)]
pub struct ScriptRunner {
    pub instructions: Vec<Instruction>,
    pub ip: usize,
    pub waiting: bool,
}

pub fn script_runner_system(
    mut runner: ResMut<ScriptRunner>,
    mut dialogue: ResMut<DialogueState>,
) {
    if runner.waiting {
        return;
    }

    if runner.ip >= runner.instructions.len() {
        return;
    }

    let instruction = runner.instructions[runner.ip].clone();

    match instruction {
        Instruction::Say(text) => {
            dialogue.current_line = Some(text);
            runner.waiting = true;
        }

        Instruction::SetVar(_, _) => {
            // handled later
        }
        Instruction::Jump(pos) => {
            runner.ip = pos;
            return;
        }
    }

    runner.ip += 1;
}

pub fn advance_dialogue(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut runner: ResMut<ScriptRunner>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        runner.waiting = false;
    }
}
