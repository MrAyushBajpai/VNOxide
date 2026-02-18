use std::fs;
use crate::script::runner::Instruction;

pub fn load_script(path: &str) -> Vec<Instruction> {
    let content = fs::read_to_string(format!("assets/{}", path))
        .expect("Failed to read script file");

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

        println!("Unknown script line: {}", line);
    }

    instructions
}
