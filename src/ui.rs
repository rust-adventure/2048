use bevy::prelude::*;

use crate::{colors, FontSpec};

mod styles;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
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
