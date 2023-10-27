use crate::colors::{BUTTON_MATERIALS, MATERIALS};
use crate::{FontSpec, Game, RunState};
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

fn setup_ui(
    mut commands: Commands,
    font_spec: Res<FontSpec>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content:
                    JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(50.0)),
                ..Default::default()
            },
            background_color: BackgroundColor(
                MATERIALS.none,
            ),
            ..Default::default()
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
                        width: Val::Auto,
                        height: Val::Auto,
                        column_gap: Val::Px(20.0),
                        row_gap: Val::Px(20.),
                        ..default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // scorebox
                    parent
                        .spawn(NodeBundle {
                            style: styles::SCORE_CONTAINER,
                            background_color:
                                BackgroundColor(
                                    MATERIALS.score_box,
                                ),
                            ..Default::default()
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
                                    MATERIALS.score_box,
                                ),
                            ..Default::default()
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
                        width: Val::Px(130.0),
                        height: Val::Px(50.0),
                        justify_content:
                            JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
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
                        ..Default::default()
                    });
                });
        });
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
            Interaction::Pressed => {
                *color = BUTTON_MATERIALS.pressed.into();

                match run_state.get() {
                    RunState::Playing => {
                        next_state.set(RunState::GameOver);
                    }
                    RunState::GameOver => {
                        next_state.set(RunState::Playing);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BUTTON_MATERIALS.hovered.into();
            }
            Interaction::None => {
                *color = BUTTON_MATERIALS.normal.into();
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
