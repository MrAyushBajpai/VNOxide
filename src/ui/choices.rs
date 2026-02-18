use bevy::prelude::*;
use crate::script::runner::ScriptRunner;

#[derive(Resource, Default)]
pub struct ChoiceRequest {
    pub options: Option<Vec<(String, String)>>, // (text, label)
}

#[derive(Component)]
pub struct ChoiceButton {
    pub target_label: String,
}

#[derive(Component)]
pub struct ChoiceRoot; // Marks the root UI container

pub fn choice_ui_system(
    mut commands: Commands,
    mut choice_req: ResMut<ChoiceRequest>,
    asset_server: Res<AssetServer>,
) {
    if choice_req.options.is_none() {
        return;
    }

    let options = choice_req.options.take().unwrap();
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(30.0),
                position_type: PositionType::Absolute,
                top: Val::Percent(30.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ChoiceRoot,
        ))
        .with_children(|parent| {
            for (text, label) in options {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(300.0),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        ChoiceButton {
                            target_label: label.clone(),
                        },
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(text),
                            TextFont {
                                font: font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

pub fn choice_click_system(
    mut interaction_query: Query<
        (&Interaction, &ChoiceButton),
        (Changed<Interaction>, With<Button>),
    >,
    root_query: Query<Entity, With<ChoiceRoot>>,
    mut commands: Commands,
    mut runner: ResMut<ScriptRunner>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            runner.jump_to_label(&button.target_label);

            // Despawn entire choice UI
            for root in &root_query {
                commands.entity(root).despawn(); // recursive by default in modern Bevy
            }
        }
    }
}
