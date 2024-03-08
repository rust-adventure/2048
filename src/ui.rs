use crate::colors;
use crate::{Game, RunState};
use bevy::prelude::*;

mod styles;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

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

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content:
                    JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            background_color: BackgroundColor(
                colors::palette::NONE,
            ),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "2048",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content:
                            JustifyContent::Center,
                        width: Val::Auto,
                        height: Val::Auto,
                        column_gap: Val::Px(20.0),
                        row_gap: Val::Px(20.),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // scorebox
                    parent
                        .spawn((
                            ImageBundle {
                                style:
                                    styles::SCORE_CONTAINER,
                                image: ui_assets
                                    .panel
                                    .clone()
                                    .into(),
                                ..default()
                            },
                            ImageScaleMode::Sliced(
                                panel_slicer.clone(),
                            ),
                        ))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Score",
                                    TextStyle {
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_text_justify(
                                    JustifyText::Center,
                                ),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_text_justify(
                                    JustifyText::Center,
                                ),
                                ScoreDisplay,
                            ));
                        });
                    // end scorebox
                    // best scorebox
                    parent
                        .spawn((
                            ImageBundle {
                                style:
                                    styles::SCORE_CONTAINER,
                                image: ui_assets
                                    .panel_green
                                    .clone()
                                    .into(),
                                ..default()
                            },
                            ImageScaleMode::Sliced(
                                panel_slicer,
                            ),
                        ))
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Best",
                                    TextStyle {
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_text_justify(
                                    JustifyText::Center,
                                ),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                )
                                .with_text_justify(
                                    JustifyText::Center,
                                ),
                                BestScoreDisplay,
                            ));
                        });
                    // end best scorebox
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(130.0),
                            height: Val::Px(50.0),
                            justify_content:
                                JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        image: ui_assets
                            .button
                            .clone()
                            .into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(slicer.clone()),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
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
                    });
                });
        });

    commands.insert_resource(ui_assets);
}

fn scoreboard(
    game: Res<Game>,
    mut query_scores: ParamSet<(
        Query<&mut Text, With<ScoreDisplay>>,
        Query<&mut Text, With<BestScoreDisplay>>,
    )>,
) {
    let mut p0 = query_scores.p0();
    let mut text = p0.single_mut();
    text.sections[0].value = game.score.to_string();

    let mut p1 = query_scores.p1();
    let mut text = p1.single_mut();
    text.sections[0].value = game.score_best.to_string();
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
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    run_state: Res<State<RunState>>,
) {
    let children = button_query.single();
    let mut text =
        text_query
            .get_mut(*children.first().expect(
                "expect button to have a first child",
            ))
            .unwrap();
    match run_state.get() {
        RunState::Playing => {
            text.sections[0].value = "End Game".to_string();
        }
        RunState::GameOver => {
            text.sections[0].value = "New Game".to_string();
        }
    }
}
