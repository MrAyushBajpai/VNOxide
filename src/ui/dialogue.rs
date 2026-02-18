use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DialogueState {
    pub current_line: Option<String>,
}

#[derive(Component)]
pub struct DialogueText;

pub fn setup_dialogue_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/main.ttf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(25.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new(""),
            TextFont {
                font,
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
            DialogueText,
        ));
    });
}

pub fn update_dialogue_text(
    dialogue: Res<DialogueState>,
    mut query: Query<&mut Text, With<DialogueText>>,
) {
    if !dialogue.is_changed() {
        return;
    }

    for mut text in &mut query {
        text.0 = dialogue.current_line.clone().unwrap_or_default();
    }
}
