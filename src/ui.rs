use crate::{Game, RunState};
use bevy::{
    color::palettes::tailwind::*,
    picking::hover::PickingInteraction, prelude::*,
};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    scoreboard,
                    button_interaction_system,
                    button_text_system,
                ),
            )
            .add_observer(
                |trigger: Trigger<Pointer<Click>>,
                 buttons: Query<
                    (),
                    With<NewGameButton>,
                >,
                 run_state: Res<State<RunState>>,
                 mut next_state: ResMut<
                    NextState<RunState>,
                >| {
                    if buttons
                        .get(trigger.target())
                        .is_err()
                    {
                        return;
                    };

                    match run_state.get() {
                        RunState::Playing => {
                            next_state
                                .set(RunState::GameOver);
                        }
                        RunState::GameOver => {
                            next_state
                                .set(RunState::Playing);
                        }
                        RunState::Startup => {}
                    }
                },
            );
    }
}

#[derive(Component)]
struct NewGameButton;

#[derive(Component)]
struct NewGameButtonText;

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    run_state: Res<State<RunState>>,
) {
    let font = asset_server.load("Outfit-Black.ttf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(50.0)),
            ..default()
        },
        children![
            (
                Text("2048".to_string()),
                TextColor(Color::WHITE),
                TextFont {
                    font: font.clone(),
                    font_size: 66.0,
                    ..default()
                },
            ),
            (
                Node {
                    align_self: AlignSelf::FlexEnd,
                    column_gap: Val::Px(10.0),
                    row_gap: Val::Px(20.),
                    height: Val::Px(75.),
                    ..default()
                },
                children![
                    scorebox(
                        "Score: ".to_string(),
                        font.clone(),
                        ScoreDisplay
                    ),
                    scorebox(
                        "Best: ".to_string(),
                        font.clone(),
                        BestScoreDisplay
                    ),
                ],
            ),
            (
                Node {
                    width: Val::Px(130.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Button,
                BackgroundColor::from(
                    match run_state.get() {
                        RunState::Playing => RED_800,
                        RunState::GameOver => BLUE_800,
                        RunState::Startup => RED_800,
                    }
                ),
                children![(
                    Text::default(),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(SLATE_50.into()),
                    Pickable::IGNORE,
                    children![(
                        TextSpan("New Game".to_string()),
                        NewGameButtonText,
                    )],
                )],
                NewGameButton,
            )
        ],
    ));
}

fn scorebox(
    text: String,
    font: Handle<Font>,
    extra: impl Bundle,
) -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            min_width: Val::Px(100.),
            ..default()
        },
        BackgroundColor(SLATE_600.into()),
        BorderRadius::all(Val::Px(10.)),
        children![
            (
                Text(text),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Text::default(),
                children![(
                    TextSpan("<score>".to_string()),
                    TextFont {
                        font: font.clone(),
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    extra,
                )],
            )
        ],
    )
}

fn scoreboard(
    game: Res<Game>,
    mut scores: Query<&mut TextSpan, With<ScoreDisplay>>,
    mut scores_best: Query<
        &mut TextSpan,
        (
            With<BestScoreDisplay>,
            Without<ScoreDisplay>,
        ),
    >,
) {
    for mut span in scores.iter_mut() {
        span.0 = game.score.to_string();
    }

    for mut span in scores_best.iter_mut() {
        span.0 = game.score_best.to_string();
    }
}

fn button_interaction_system(
    mut interaction_query: Query<
        (
            &PickingInteraction,
            &mut BackgroundColor,
        ),
        (
            Changed<PickingInteraction>,
            With<Button>,
        ),
    >,
    run_state: Res<State<RunState>>,
) {
    for (interaction, mut background_color) in
        &mut interaction_query
    {
        match (interaction, run_state.get()) {
            (_, RunState::Startup) => {}
            (
                PickingInteraction::Pressed,
                RunState::Playing,
            ) => {
                *background_color = RED_900.into();
            }
            (
                PickingInteraction::Pressed,
                RunState::GameOver,
            ) => {
                *background_color = BLUE_900.into();
            }
            (
                PickingInteraction::Hovered,
                RunState::Playing,
            ) => {
                *background_color = RED_700.into();
            }
            (
                PickingInteraction::Hovered,
                RunState::GameOver,
            ) => {
                *background_color = BLUE_700.into();
            }
            (
                PickingInteraction::None,
                RunState::Playing,
            ) => {
                *background_color = RED_800.into();
            }
            (
                PickingInteraction::None,
                RunState::GameOver,
            ) => {
                *background_color = BLUE_800.into();
            }
        }
    }
}

fn button_text_system(
    mut text_query: Query<
        &mut TextSpan,
        With<NewGameButtonText>,
    >,
    run_state: Res<State<RunState>>,
) {
    let new_text = match run_state.get() {
        RunState::Playing | RunState::Startup => {
            "End Game".to_string()
        }
        RunState::GameOver => "New Game".to_string(),
    };

    for mut span in &mut text_query {
        span.0 = new_text.clone();
    }
}
