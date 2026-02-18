use bevy::prelude::*;

mod scene;
mod script;
mod vars;
mod save;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<vars::store::VarStore>()
        .init_resource::<script::runner::ScriptRunner>()
        .init_resource::<ui::dialogue::DialogueState>()
        .add_systems(Startup, (
            scene::loader::load_test_scene,
            ui::dialogue::setup_dialogue_ui,
        ))
        .add_systems(Update, (
            script::runner::script_runner_system,
            script::runner::advance_dialogue,
            ui::dialogue::update_dialogue_text,
        ))
        .run();
}
