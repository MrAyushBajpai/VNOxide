use std::fs;
use crate::script::runner::Instruction;

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
            if let Some((cond, target)) = rest.split_once(" jump ") {
                instructions.push(Instruction::IfJump {
                    condition: cond.trim().to_string(),
                    target: target.trim().to_string(),
                });
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("show ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 3 {
                instructions.push(Instruction::ShowCharacter {
                    name: parts[0].to_string(),
                    expression: parts[1].to_string(),
                    position: parts[2].to_string(),
                });
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("hide ") {
            instructions.push(Instruction::HideCharacter {
                name: rest.trim().to_string(),
            });
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

        println!("Unknown line: {}", line);
    }

    instructions
}
