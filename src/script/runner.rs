use bevy::prelude::*;
use std::collections::HashMap;

use crate::ui::dialogue::DialogueState;
use crate::ui::choices::ChoiceRequest;
use crate::vars::store::{VarStore, Value};

#[derive(Debug, Clone)]
pub enum Instruction {
    Say(String),
    Label(String),
    JumpLabel(String),

    // set x = 5, set x += 2 etc
    SetVar {
        name: String,
        op: SetOp,
        value: f64,
    },

    // if x >= 5 jump label
    IfJump {
        var: String,
        cmp: CmpOp,
        value: f64,
        target: String,
    },

    Choice(Vec<(String, String)>),
}

#[derive(Debug, Clone)]
pub enum SetOp {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum CmpOp {
    Eq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
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
        } else {
            println!("Label not found: {}", label);
        }
    }
}

fn get_number(vars: &VarStore, name: &str) -> f64 {
    match vars.get(name) {
        Some(Value::Int(v)) => *v as f64,
        Some(Value::Float(v)) => *v as f64,
        _ => 0.0,
    }
}

fn set_number(vars: &mut VarStore, name: &str, value: f64) {
    vars.set(name, Value::Float(value as f32));
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

        Instruction::Label(_) => {}

        Instruction::JumpLabel(label) => {
            runner.jump_to_label(&label);
            return;
        }

        Instruction::Choice(options) => {
            choice_req.options = Some(options);
            runner.waiting = true;
        }

        Instruction::SetVar { name, op, value } => {
            let current = get_number(&vars, &name);

            let result = match op {
                SetOp::Assign => value,
                SetOp::Add => current + value,
                SetOp::Sub => current - value,
                SetOp::Mul => current * value,
                SetOp::Div => current / value,
                SetOp::Mod => current % value,
            };

            set_number(&mut vars, &name, result);
        }

        Instruction::IfJump { var, cmp, value, target } => {
            let current = get_number(&vars, &var);

            let condition = match cmp {
                CmpOp::Eq => current == value,
                CmpOp::Greater => current > value,
                CmpOp::Less => current < value,
                CmpOp::GreaterEq => current >= value,
                CmpOp::LessEq => current <= value,
            };

            if condition {
                runner.jump_to_label(&target);
                return;
            }
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
