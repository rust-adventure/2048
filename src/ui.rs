use bevy::prelude::*;

use crate::{colors, FontSpec, Game, RunState};

mod styles;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui).add_systems((
            scoreboard,
            button_interaction_system,
            button_text_system,
        ));
    }
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

fn setup_ui(
    mut commands: Commands,
    font_spec: Res<FontSpec>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(
                    Val::Percent(100.0),
                    Val::Percent(100.0),
                ),
                align_items: AlignItems::FlexStart,
                justify_content:
                    JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "2048",
                TextStyle {
                    font: font_spec.family.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content:
                            JustifyContent::Center,
                        size: Size::AUTO,
                        gap: Size::all(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // scorebox
                    parent
                        .spawn(NodeBundle {
                            style: styles::SCORE_CONTAINER,
                            background_color:
                                BackgroundColor(
                                    colors::SCORE_BOX,
                                ),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Score",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                                ScoreDisplay,
                            ));
                        });
                    // end scorebox
                    // best scorebox
                    parent
                        .spawn(NodeBundle {
                            style: styles::SCORE_CONTAINER,
                            background_color:
                                BackgroundColor(
                                    colors::SCORE_BOX,
                                ),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Best",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                            );
                            parent.spawn((
                                TextBundle::from_section(
                                    "<score>",
                                    TextStyle {
                                        font: font_spec
                                            .family
                                            .clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_text_alignment(
                                    TextAlignment::Center,
                                ),
                                BestScoreDisplay,
                            ));
                        });
                    // end best scorebox
                });

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(
                            Val::Px(130.0),
                            Val::Px(50.0),
                        ),
                        justify_content:
                            JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Button",
                            TextStyle {
                                font: font_spec
                                    .family
                                    .clone(),
                                font_size: 20.0,
                                color: Color::rgb(
                                    0.9, 0.9, 0.9,
                                ),
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn scoreboard(
    game: Res<Game>,
    mut query_scores: Query<
        &mut Text,
        (
            With<ScoreDisplay>,
            Without<BestScoreDisplay>,
        ),
    >,
    mut query_best_scores: Query<
        &mut Text,
        (
            With<BestScoreDisplay>,
            Without<ScoreDisplay>,
        ),
    >,
) {
    let mut text = query_scores.single_mut();
    text.sections[0].value = game.score.to_string();

    let mut text = query_best_scores.single_mut();
    text.sections[0].value = game.score_best.to_string();
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    run_state: Res<State<RunState>>,
    mut next_state: ResMut<NextState<RunState>>,
) {
    for (interaction, mut color) in
        interaction_query.iter_mut()
    {
        match interaction {
            Interaction::Clicked => {
                *color = colors::button::PRESSED.into();

                match run_state.0 {
                    RunState::Playing => {
                        next_state.set(RunState::GameOver);
                    }
                    RunState::GameOver => {
                        next_state.set(RunState::Playing);
                    }
                }
            }
            Interaction::Hovered => {
                *color = colors::button::HOVERED.into();
            }
            Interaction::None => {
                *color = colors::button::NORMAL.into();
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
    let first_child_entity = children
        .first()
        .expect("expect button to have a first child");
    let mut text =
        text_query.get_mut(*first_child_entity).unwrap();
    match run_state.0 {
        RunState::Playing => {
            text.sections[0].value = "End Game".to_string();
        }
        RunState::GameOver => {
            text.sections[0].value = "New Game".to_string();
        }
    }
}
