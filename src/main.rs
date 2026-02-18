use bevy::prelude::*;

mod scene;
mod script;
mod vars;
mod save;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<vars::store::VarStore>()
        .init_resource::<script::runner::ScriptRunner>()
        .add_systems(Startup, scene::loader::load_test_scene)
        .add_systems(Update, (
            script::runner::script_runner_system,
            script::runner::advance_dialogue,
        ))
        .run();
}
