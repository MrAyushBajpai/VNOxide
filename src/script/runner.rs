use bevy::prelude::*;
use std::collections::HashMap;

use crate::ui::dialogue::DialogueState;
use crate::ui::choices::{ChoiceRequest};
use crate::vars::store::{VarStore, Value};

#[derive(Debug, Clone)]
pub enum Instruction {
    Say(String),
    SetVar(String, i64),
    Label(String),
    JumpLabel(String),
    Choice(Vec<(String, String)>), // (button text, target label)
}

#[derive(Resource)]
pub struct ScriptRunner {
    pub instructions: Vec<Instruction>,
    pub labels: HashMap<String, usize>,
    pub ip: usize,
    pub waiting: bool,
}

impl Default for ScriptRunner {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            ip: 0,
            waiting: false,
        }
    }
}

impl ScriptRunner {
    pub fn rebuild_labels(&mut self) {
        self.labels.clear();

        for (index, instr) in self.instructions.iter().enumerate() {
            if let Instruction::Label(name) = instr {
                self.labels.insert(name.clone(), index);
            }
        }
    }

    pub fn jump_to_label(&mut self, label: &str) {
        if let Some(&pos) = self.labels.get(label) {
            self.ip = pos;
            self.waiting = false;
        } else {
            println!("Label not found: {}", label);
        }
    }
}

pub fn script_runner_system(
    mut runner: ResMut<ScriptRunner>,
    mut dialogue: ResMut<DialogueState>,
    mut vars: ResMut<VarStore>,
    mut choice_req: ResMut<ChoiceRequest>,
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

        Instruction::SetVar(name, value) => {
            vars.set(&name, Value::Int(value));
        }

        Instruction::Label(_) => {}

        Instruction::JumpLabel(label) => {
            runner.jump_to_label(&label);
            return;
        }

        Instruction::Choice(options) => {
            choice_req.options = Some(options);
            runner.waiting = true;
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
