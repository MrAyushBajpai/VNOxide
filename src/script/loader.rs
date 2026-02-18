use std::fs;
use crate::script::runner::{Instruction, CmpOp};

pub fn load_script(path: &str) -> Vec<Instruction> {
    let content = fs::read_to_string(format!("assets/{}", path))
        .expect("Failed to read script");

    let mut instructions = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix("label ") {
            instructions.push(Instruction::Label(rest.trim().to_string()));
            continue;
        }

        if let Some(rest) = line.strip_prefix("say ") {
            instructions.push(Instruction::Say(rest.trim().to_string()));
            continue;
        }

        if let Some(rest) = line.strip_prefix("jump ") {
            instructions.push(Instruction::JumpLabel(rest.trim().to_string()));
            continue;
        }

        if let Some(rest) = line.strip_prefix("set ") {
            if let Some((name, expr)) = rest.split_once('=') {
                instructions.push(Instruction::SetVar {
                    name: name.trim().to_string(),
                    expression: expr.trim().to_string(),
                });
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("if ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() == 5 && parts[3] == "jump" {
                let var = parts[0].to_string();
                let value: f64 = parts[2].parse().unwrap_or(0.0);
                let target = parts[4].to_string();

                let cmp = match parts[1] {
                    "==" => CmpOp::Eq,
                    ">" => CmpOp::Greater,
                    "<" => CmpOp::Less,
                    ">=" => CmpOp::GreaterEq,
                    "<=" => CmpOp::LessEq,
                    _ => continue,
                };

                instructions.push(Instruction::IfJump {
                    var,
                    cmp,
                    value,
                    target,
                });
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("choice ") {
            let mut options = Vec::new();

            for part in rest.split('|') {
                if let Some((text, label)) = part.split_once("->") {
                    options.push((
                        text.trim().to_string(),
                        label.trim().to_string(),
                    ));
                }
            }

            instructions.push(Instruction::Choice(options));
            continue;
        }
    }

    instructions
}
