use crate::{Game, RunState};
use bevy::{
    color::palettes::tailwind::*,
    picking::focus::PickingInteraction, prelude::*,
};

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

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    run_state: Res<State<RunState>>,
) {
    let font = asset_server.load("Outfit-Black.ttf");

    let score_box = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                min_width: Val::Px(100.),
                ..default()
            },
            BackgroundColor(SLATE_600.into()),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_child((
            Text("Score ".to_string()),
            TextFont {
                font: font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .with_children(|parent| {
            parent.spawn(Text::default()).with_child((
                TextSpan("<score>".to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreDisplay,
            ));
        })
        .id();

    let highscore_box = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                min_width: Val::Px(100.),
                ..default()
            },
            BackgroundColor(SLATE_700.into()),
            BorderRadius::all(Val::Px(10.)),
        ))
        .with_child((
            Text("Best".to_string()),
            TextFont {
                font: font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .with_children(|parent| {
            parent.spawn(Text::default()).with_child((
                TextSpan("<score>".to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                BestScoreDisplay,
            ));
        })
        .id();

    let scorebox_container = commands
        .spawn(Node {
            align_self: AlignSelf::FlexEnd,
            column_gap: Val::Px(10.0),
            row_gap: Val::Px(20.),
            height: Val::Px(75.),
            ..default()
        })
        .add_child(score_box)
        .add_child(highscore_box)
        .id();

    let new_game_button = commands
        .spawn((
            Node {
                width: Val::Px(130.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Button,
            BackgroundColor::from(match run_state.get() {
                RunState::Playing => RED_800,
                RunState::GameOver => BLUE_800,
                RunState::Startup => RED_800,
            }),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::default(),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(SLATE_50.into()),
                    PickingBehavior::IGNORE,
                ))
                .with_child((
                    TextSpan("New Game".to_string()),
                    NewGameButtonText,
                ));
        })
        .observe(
            |_trigger: Trigger<Pointer<Click>>,
             run_state: Res<State<RunState>>,
             mut next_state: ResMut<
                NextState<RunState>,
            >| {
                match run_state.get() {
                    RunState::Playing => {
                        next_state.set(RunState::GameOver);
                    }
                    RunState::GameOver => {
                        next_state.set(RunState::Playing);
                    }
                    RunState::Startup => {}
                }
            },
        )
        .id();

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(50.0)),
            ..default()
        })
        .with_child((
            Text("2048".to_string()),
            TextColor(Color::WHITE),
            TextFont {
                font: font.clone(),
                font_size: 66.0,
                ..default()
            },
        ))
        .add_child(scorebox_container)
        .add_child(new_game_button);
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
    let Ok(mut span) = text_query.get_single_mut() else {
        error!("Expected a single NewGameButtonText");
        return;
    };

    let new_text = match run_state.get() {
        RunState::Playing | RunState::Startup => {
            "End Game".to_string()
        }
        RunState::GameOver => "New Game".to_string(),
    };

    span.0 = new_text;
}
