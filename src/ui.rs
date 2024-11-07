use crate::{Game, RunState};
use bevy::{color::palettes::tailwind::*, prelude::*};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (scoreboard, button_text_system),
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
    font: Handle<Font>,
}

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    run_state: Res<State<RunState>>,
) {
    let ui_assets = UiAssets {
        button_red: asset_server.load("button_red.png"),
        button: asset_server.load("button.png"),
        font: asset_server.load("Outfit-Black.ttf"),
    };

    let slicer = TextureSlicer {
        border: BorderRect::square(15.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

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
                font: ui_assets.font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .with_children(|parent| {
            parent.spawn(Text::default()).with_child((
                TextSpan("<score>".to_string()),
                TextFont {
                    font: ui_assets.font.clone(),
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
                font: ui_assets.font.clone(),
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .with_children(|parent| {
            parent.spawn(Text::default()).with_child((
                TextSpan("<score>".to_string()),
                TextFont {
                    font: ui_assets.font.clone(),
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
            UiImage::from(match run_state.get() {
                RunState::Playing => {
                    ui_assets.button_red.clone()
                }

                RunState::GameOver => {
                    ui_assets.button.clone()
                }
                RunState::Startup => {
                    ui_assets.button_red.clone()
                }
            }),
            ImageScaleMode::Sliced(slicer.clone()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::default(),
                    TextFont {
                        font: ui_assets.font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    PickingBehavior::IGNORE,
                ))
                .with_child((
                    TextSpan("New Game".to_string()),
                    NewGameButtonText,
                ));
        })
        .observe(
            |trigger: Trigger<Pointer<Over>>,
             mut images: Query<&mut UiImage>,
             run_state: Res<State<RunState>>,
             ui_assets: Res<UiAssets>| {
                let mut image =
                    images.get_mut(trigger.target).unwrap();
                match run_state.get() {
                    RunState::Playing => {
                        *image = ui_assets
                            .button_red
                            .clone()
                            .into();
                        // tint button slightly darker
                        image.color =
                            Color::srgb(0.9, 0.9, 0.9);
                    }
                    RunState::GameOver => {
                        *image =
                            ui_assets.button.clone().into();
                        // tint button slightly darker
                        image.color =
                            Color::srgb(0.9, 0.9, 0.9);
                    }
                    RunState::Startup => {}
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Out>>,
             mut images: Query<&mut UiImage>,
             run_state: Res<State<RunState>>,
             ui_assets: Res<UiAssets>| {
                let mut image =
                    images.get_mut(trigger.target).unwrap();
                match run_state.get() {
                    RunState::Playing => {
                        *image = ui_assets
                            .button_red
                            .clone()
                            .into();
                    }

                    RunState::GameOver => {
                        *image =
                            ui_assets.button.clone().into();
                    }
                    RunState::Startup => {}
                }
            },
        )
        .observe(
            |trigger: Trigger<Pointer<Click>>,
             mut images: Query<&mut UiImage>,
             run_state: Res<State<RunState>>,
             mut next_state: ResMut<
                NextState<RunState>,
            >,
             ui_assets: Res<UiAssets>| {
                let mut image =
                    images.get_mut(trigger.target).unwrap();
                match run_state.get() {
                    RunState::Playing => {
                        *image =
                            ui_assets.button.clone().into();
                        next_state.set(RunState::GameOver);
                    }
                    RunState::GameOver => {
                        *image = ui_assets
                            .button_red
                            .clone()
                            .into();
                        next_state.set(RunState::Playing);
                    }
                    RunState::Startup => {}
                }
            },
        )
        .id();

    commands
        .spawn(Node {
            align_items: AlignItems::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(50.0)),
            ..default()
        })
        .with_child((
            Text("2048".to_string()),
            TextColor(Color::WHITE),
            TextFont {
                font: ui_assets.font.clone(),
                font_size: 66.0,
                ..default()
            },
        ))
        .add_child(new_game_button);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexEnd,
            padding: UiRect::all(Val::Px(50.0)),
            ..default()
        })
        .add_child(scorebox_container);

    commands.insert_resource(ui_assets);
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
