use bevy::prelude::*;
use std::collections::HashMap;

use crate::ui::dialogue::DialogueState;
use crate::ui::choices::ChoiceRequest;
use crate::vars::store::{VarStore, Value};
use crate::script::expr::eval;
use crate::scene::characters::{
    CharacterManager,
    show_character,
    hide_character,
    TransformParams,
};

#[derive(Debug, Clone)]
pub enum Instruction {
    Say(String),
    Label(String),
    JumpLabel(String),

    SetVar {
        name: String,
        expression: String,
    },

    IfJump {
        condition: String,
        target: String,
    },

    Choice(Vec<(String, String)>),

    ShowCharacter {
        name: String,
        expression: String,
        params: TransformParams,
    },

    HideCharacter {
        name: String,
    },
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
        for (i, instr) in self.instructions.iter().enumerate() {
            if let Instruction::Label(name) = instr {
                self.labels.insert(name.clone(), i);
            }
        }
    }

    pub fn jump_to_label(&mut self, label: &str) {
        if let Some(&pos) = self.labels.get(label) {
            self.ip = pos;
            self.waiting = false;
        }
    }
}

fn set_number(vars: &mut VarStore, name: &str, value: f64) {
    vars.set(name, Value::Float(value as f32));
}

pub fn script_runner_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut runner: ResMut<ScriptRunner>,
    mut dialogue: ResMut<DialogueState>,
    mut vars: ResMut<VarStore>,
    mut choice_req: ResMut<ChoiceRequest>,
    mut characters: ResMut<CharacterManager>,
) {
    if runner.waiting || runner.ip >= runner.instructions.len() {
        return;
    }

    let instruction = runner.instructions[runner.ip].clone();

    match instruction {
        Instruction::Say(text) => {
            dialogue.current_line = Some(text);
            runner.waiting = true;
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

        Instruction::SetVar { name, expression } => {
            let value = eval(&expression, &vars);
            set_number(&mut vars, &name, value);
        }

        Instruction::IfJump { condition, target } => {
            if eval(&condition, &vars) != 0.0 {
                runner.jump_to_label(&target);
                return;
            }
        }

        Instruction::ShowCharacter { name, expression, params } => {
            show_character(
                &mut commands,
                &asset_server,
                &mut characters,
                name,
                expression,
                params,
            );
        }

        Instruction::HideCharacter { name } => {
            hide_character(&mut commands, &mut characters, &name);
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
