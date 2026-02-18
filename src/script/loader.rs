use std::fs;
use crate::script::runner::Instruction;
use crate::scene::characters::TransformParams;

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

            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let expression = parts[1].to_string();

                let mut params = TransformParams::default();

                for part in &parts[2..] {
                    if *part == "left" || *part == "center" || *part == "right" {
                        params.preset = Some(part.to_string());
                    } else if let Some((k, v)) = part.split_once('=') {
                        match k {
                            "x" => params.x = v.parse().ok(),
                            "y" => params.y = v.parse().ok(),
                            "scale" => params.scale = v.parse().ok(),
                            "rot" => params.rotation_deg = v.parse().ok(),
                            _ => {}
                        }
                    }
                }

                instructions.push(Instruction::ShowCharacter {
                    name,
                    expression,
                    params,
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

        println!("Unknown line: {}", line);
    }

    instructions
}
