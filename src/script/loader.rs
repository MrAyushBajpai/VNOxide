use std::fs;
use bevy::prelude::Color;

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

        if let Some(rest) = line.strip_prefix("music play ") {
            instructions.push(Instruction::MusicPlay(rest.trim().to_string()));
            continue;
        }

        if line == "music stop" {
            instructions.push(Instruction::MusicStop);
            continue;
        }

        if let Some(rest) = line.strip_prefix("sfx play ") {
            instructions.push(Instruction::SfxPlay(rest.trim().to_string()));
            continue;
        }

        if let Some(rest) = line.strip_prefix("bg ") {
            if let Some(path) = rest.strip_prefix("image=") {
                instructions.push(Instruction::BgImage(path.trim().to_string()));
                continue;
            }
        }

        if let Some(rest) = line.strip_prefix("show ") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let expression = parts[1].to_string();
                let params = TransformParams::default();

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
    }

    instructions
}
