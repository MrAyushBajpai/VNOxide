use std::fs;
use bevy::prelude::Color;

use crate::script::runner::Instruction;
use crate::scene::characters::TransformParams;

fn parse_color(value: &str) -> Option<Color> {
    match value {
        "red" => Some(Color::srgb(1.0, 0.0, 0.0)),
        "green" => Some(Color::srgb(0.0, 1.0, 0.0)),
        "blue" => Some(Color::srgb(0.0, 0.0, 1.0)),
        _ => {
            if value.starts_with('#') && value.len() == 7 {
                let r = u8::from_str_radix(&value[1..3], 16).ok()?;
                let g = u8::from_str_radix(&value[3..5], 16).ok()?;
                let b = u8::from_str_radix(&value[5..7], 16).ok()?;
                Some(Color::srgb_u8(r, g, b))
            } else {
                None
            }
        }
    }
}

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

        if let Some(rest) = line.strip_prefix("bg ") {
            if let Some(path) = rest.strip_prefix("image=") {
                instructions.push(Instruction::BgImage(path.trim().to_string()));
                continue;
            }

            if let Some(color_str) = rest.strip_prefix("color=") {
                if let Some(color) = parse_color(color_str.trim()) {
                    instructions.push(Instruction::BgColor(color));
                }
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
