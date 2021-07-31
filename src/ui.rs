use crate::{FontSpec, Game, Materials, RunState};
use bevy::prelude::*;

pub struct ScoreDisplay;
pub struct BestScoreDisplay;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_startup_system(setup_ui.system())
            .add_system(scoreboard.system())
            .add_system(button_interaction_system.system())
            .add_system(button_text_system.system());
    }
}

fn setup_ui(
    mut commands: Commands,
    materials: Res<Materials>,
    font_spec: Res<FontSpec>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::FlexEnd,
                padding: Rect::all(Val::Px(50.0)),
                ..Default::default()
            },
            material: materials.none.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "2048",
                    TextStyle {
                        font: font_spec.family.clone(),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(100.0), Val::Auto),
                        ..Default::default()
                    },
                    material: materials.none.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // scorebox
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                margin: Rect {
                                    left: Val::Px(20.0),
                                    right: Val::Px(20.0),
                                    top: Val::Px(0.0),
                                    bottom: Val::Px(0.0),
                                },
                                padding: Rect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            material: materials.tile_placeholder.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Score",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "<score>",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(ScoreDisplay);
                        });
                    // end scorebox
                    // best scorebox
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                padding: Rect::all(Val::Px(10.0)),
                                ..Default::default()
                            },
                            material: materials.tile_placeholder.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Best",
                                    TextStyle {
                                        font: font_spec.family.clone(),
                                        font_size: 15.0,
                                        color: Color::WHITE,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        "<score>",
                                        TextStyle {
                                            font: font_spec.family.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                        TextAlignment {
                                            vertical: VerticalAlign::Center,
                                            horizontal: HorizontalAlign::Center,
                                        },
                                    ),
                                    ..Default::default()
                                })
                                .insert(BestScoreDisplay);
                        });
                    // end best scorebox
                });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Px(30.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Button",
                            TextStyle {
                                font: font_spec.family.clone(),
                                font_size: 20.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn scoreboard(
    game: Res<Game>,
    mut query_scores: Query<&mut Text, With<ScoreDisplay>>,
) {
    let mut text = query_scores.single_mut().unwrap();
    text.sections[0].value = game.score.to_string();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .unwrap();
        ButtonMaterials {
            normal: materials
                .add(Color::rgb(0.75, 0.75, 0.9).into()),
            hovered: materials
                .add(Color::rgb(0.7, 0.7, 0.9).into()),
            pressed: materials
                .add(Color::rgb(0.6, 0.6, 1.0).into()),
        }
    }
}

fn button_interaction_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
    mut run_state: ResMut<State<RunState>>,
) {
    for (interaction, mut material) in
        interaction_query.iter_mut()
    {
        match interaction {
            Interaction::Clicked => {
                *material =
                    button_materials.pressed.clone();

                match run_state.current() {
                    RunState::Playing => {
                        run_state
                            .set(RunState::GameOver)
                            .unwrap();
                    }
                    RunState::GameOver => {
                        run_state
                            .set(RunState::Playing)
                            .unwrap();
                    }
                }
            }
            Interaction::Hovered => {
                *material =
                    button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn button_text_system(
    button_query: Query<&Children, With<Button>>,
    mut text_query: Query<&mut Text>,
    run_state: Res<State<RunState>>,
) {
    let children = button_query
        .single()
        .expect("expected only one button");
    let mut text =
        text_query
            .get_mut(*children.first().expect(
                "expect button to have a first child",
            ))
            .unwrap();
    match run_state.current() {
        RunState::Playing => {
            text.sections[0].value = "End Game".to_string();
        }
        RunState::GameOver => {
            text.sections[0].value = "New Game".to_string();
        }
    }
}
