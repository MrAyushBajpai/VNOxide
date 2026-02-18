use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DialogueState {
    pub speaker: Option<String>,
    pub current_line: Option<String>,
}

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct SpeakerText;

#[derive(Component)]
pub struct DialogueRoot;

pub fn setup_dialogue_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/main.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(25.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                padding: UiRect::all(Val::Px(12.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            DialogueRoot,
        ))
        .with_children(|parent| {

            // Speaker name
            parent.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.4)),
                SpeakerText,
            ));

            // Dialogue text
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
    mut text_query: Query<&mut Text, With<DialogueText>>,
    mut speaker_query: Query<&mut Text, (With<SpeakerText>, Without<DialogueText>)>,
) {
    if !dialogue.is_changed() {
        return;
    }

    for mut text in &mut text_query {
        text.0 = dialogue
            .current_line
            .clone()
            .unwrap_or_default();
    }

    for mut speaker in &mut speaker_query {
        speaker.0 = dialogue
            .speaker
            .clone()
            .unwrap_or_default();
    }
}
