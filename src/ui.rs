use crate::{Game, RunState};
use bevy::prelude::*;

mod styles;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (
                scoreboard,
                button_interaction_system,
                button_text_system,
            ),
        );
    }
}

#[derive(Component)]
struct NewGameButtonText;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

#[derive(Resource)]
struct UiAssets {
    button_red: Handle<Image>,
    button: Handle<Image>,
    panel: Handle<Image>,
    panel_green: Handle<Image>,
}

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let ui_assets = UiAssets {
        button_red: asset_server.load("button_red.png"),
        button: asset_server.load("button.png"),
        panel_green: asset_server.load("panel_green.png"),
        panel: asset_server.load("panel.png"),
    };

    let slicer = TextureSlicer {
        border: BorderRect::square(10.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let panel_slicer = TextureSlicer {
        border: BorderRect::square(20.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let title = commands
        .spawn(TextBundle::from_section(
            "2048",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ))
        .id();

    let score_box = commands
        .spawn((
            ImageBundle {
                style: styles::SCORE_CONTAINER,
                image: ui_assets.panel.clone().into(),
                ..default()
            },
            ImageScaleMode::Sliced(panel_slicer.clone()),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Score",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
            parent.spawn((
                TextBundle::from_section(
                    "<score>",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
                ScoreDisplay,
            ));
        })
        .id();

    let highscore_box = commands
        .spawn((
            ImageBundle {
                style: styles::SCORE_CONTAINER,
                image: ui_assets.panel_green.clone().into(),
                ..default()
            },
            ImageScaleMode::Sliced(panel_slicer),
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Best",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
            parent.spawn((
                TextBundle::from_section(
                    "<score>",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
                BestScoreDisplay,
            ));
        })
        .id();

    let scorebox_container = commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                row_gap: Val::Px(20.),
                ..default()
            },
            ..default()
        })
        .add_child(score_box)
        .add_child(highscore_box)
        .id();

    let new_game_button = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(130.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: ui_assets.button.clone().into(),
                ..default()
            },
            ImageScaleMode::Sliced(slicer.clone()),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Button",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(
                                0.9, 0.9, 0.9,
                            ),
                            ..default()
                        },
                    ),
                    ..default()
                },
                NewGameButtonText,
            ));
        })
        .id();

    commands
        .spawn(NodeBundle {
            style: Style {
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            ..default()
        })
        .add_child(title)
        .add_child(new_game_button);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            ..default()
        })
        .add_child(scorebox_container);

    commands.insert_resource(ui_assets);
}

fn scoreboard(
    game: Res<Game>,
    mut query_scores: ParamSet<(
        Query<&mut Text, With<ScoreDisplay>>,
        Query<&mut Text, With<BestScoreDisplay>>,
    )>,
) {
    for mut text in query_scores.p0().iter_mut() {
        text.sections[0].value = game.score.to_string();
    }

    for mut text in query_scores.p1().iter_mut() {
        text.sections[0].value =
            game.score_best.to_string();
    }
}

fn button_interaction_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut UiImage,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    run_state: Res<State<RunState>>,
    mut next_state: ResMut<NextState<RunState>>,
    ui_assets: Res<UiAssets>,
) {
    for (interaction, mut color, mut image) in
        interaction_query.iter_mut()
    {
        *color = Color::WHITE.into();
        match (interaction, run_state.get()) {
            (Interaction::Pressed, RunState::Playing) => {
                *image = ui_assets.button.clone().into();
                next_state.set(RunState::GameOver);
            }
            (Interaction::Pressed, RunState::GameOver) => {
                *image =
                    ui_assets.button_red.clone().into();
                next_state.set(RunState::Playing);
            }
            (Interaction::Hovered, RunState::Playing) => {
                *color = Color::WHITE.with_a(0.8).into();
                *image =
                    ui_assets.button_red.clone().into();
            }
            (Interaction::Hovered, RunState::GameOver) => {
                *color = Color::WHITE.with_a(0.8).into();
                *image = ui_assets.button.clone().into();
            }
            (Interaction::None, RunState::Playing) => {
                *image =
                    ui_assets.button_red.clone().into();
            }
            (Interaction::None, RunState::GameOver) => {
                *image = ui_assets.button.clone().into();
            }
        }
    }
}

fn button_text_system(
    mut text_query: Query<
        &mut Text,
        With<NewGameButtonText>,
    >,
    run_state: Res<State<RunState>>,
) {
    let Ok(mut text) = text_query.get_single_mut() else {
        error!("Expected a single NewGameButtonText");
        return;
    };

    let new_text = match run_state.get() {
        RunState::Playing => "End Game".to_string(),
        RunState::GameOver => "New Game".to_string(),
    };

    text.sections[0].value = new_text;
}
