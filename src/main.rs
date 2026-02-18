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
        .init_resource::<ui::choices::ChoiceRequest>()
        .add_systems(Startup, (
            scene::loader::load_test_scene,
            ui::dialogue::setup_dialogue_ui,
        ))
        .add_systems(Update, (
            script::runner::script_runner_system,
            script::runner::advance_dialogue,
            ui::dialogue::update_dialogue_text,
            ui::choices::choice_ui_system,
            ui::choices::choice_click_system,
        ))
        .run();
}
