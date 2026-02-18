use std::fs;
use crate::script::runner::{Instruction, SetOp, CmpOp};

pub fn load_script(path: &str) -> Vec<Instruction> {
    let content = fs::read_to_string(format!("assets/{}", path))
        .expect("Failed to read script");

    let mut instructions = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        // label
        if let Some(rest) = line.strip_prefix("label ") {
            instructions.push(Instruction::Label(rest.trim().to_string()));
            continue;
        }

        // say
        if let Some(rest) = line.strip_prefix("say ") {
            instructions.push(Instruction::Say(rest.trim().to_string()));
            continue;
        }

        // jump
        if let Some(rest) = line.strip_prefix("jump ") {
            instructions.push(Instruction::JumpLabel(rest.trim().to_string()));
            continue;
        }

        // set
        if let Some(rest) = line.strip_prefix("set ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() == 3 {
                let name = parts[0].to_string();
                let op_str = parts[1];
                let value: f64 = parts[2].parse().unwrap_or(0.0);

                let op = match op_str {
                    "=" => SetOp::Assign,
                    "+=" => SetOp::Add,
                    "-=" => SetOp::Sub,
                    "*=" => SetOp::Mul,
                    "/=" => SetOp::Div,
                    "%=" => SetOp::Mod,
                    _ => continue,
                };

                instructions.push(Instruction::SetVar { name, op, value });
            }
            continue;
        }

        // if condition
        if let Some(rest) = line.strip_prefix("if ") {
            // format: if x >= 5 jump label
            let parts: Vec<&str> = rest.split_whitespace().collect();

            if parts.len() == 5 && parts[3] == "jump" {
                let var = parts[0].to_string();
                let cmp_str = parts[1];
                let value: f64 = parts[2].parse().unwrap_or(0.0);
                let target = parts[4].to_string();

                let cmp = match cmp_str {
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

        // choice
        if let Some(rest) = line.strip_prefix("choice ") {
            let mut options = Vec::new();

            for part in rest.split('|') {
                let part = part.trim();
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

        println!("Unknown line: {}", line);
    }

    instructions
}
