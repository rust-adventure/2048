use crate::{Game, RunState};
use bevy::prelude::*;

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
    font: Handle<Font>,
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
        font: asset_server.load("Outfit-Black.ttf"),
    };

    let slicer = TextureSlicer {
        border: BorderRect::square(15.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let panel_slicer = TextureSlicer {
        border: BorderRect::square(40.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let title = commands
        .spawn((
            Text("2048".to_string()),
            TextColor(Color::WHITE),
            TextFont {
                font: ui_assets.font.clone(),
                font_size: 66.0,
                ..default()
            },
        ))
        .id();

    let score_box = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                min_width: Val::Px(100.),
                ..default()
            },
            UiImage::from(ui_assets.panel.clone()),
            ImageScaleMode::Sliced(panel_slicer.clone()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Score".to_string()),
                TextFont {
                    font: ui_assets.font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
            parent.spawn((
                Text("<score>".to_string()),
                TextFont {
                    font: ui_assets.font.clone(),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
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
            UiImage::from(ui_assets.panel_green.clone()),
            ImageScaleMode::Sliced(panel_slicer.clone()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Best".to_string()),
                TextFont {
                    font: ui_assets.font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
            parent.spawn((
                Text("<score>".to_string()),
                TextFont {
                    font: ui_assets.font.clone(),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
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
            UiImage::from(ui_assets.button.clone()),
            ImageScaleMode::Sliced(slicer.clone()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Button".to_string()),
                TextFont {
                    font: ui_assets.font.clone(),

                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                NewGameButtonText,
            ));
        })
        .id();

    commands
        .spawn(Node {
            align_items: AlignItems::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(50.0)),
            ..default()
        })
        .add_child(title)
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
    mut query_scores: ParamSet<(
        Query<Entity, With<ScoreDisplay>>,
        Query<Entity, With<BestScoreDisplay>>,
    )>,
    mut writer: TextUiWriter,
) {
    for text_entity in query_scores.p0().iter_mut() {
        *writer.text(text_entity, 0) =
            game.score.to_string();
    }

    for text_entity in query_scores.p1().iter_mut() {
        *writer.text(text_entity, 0) =
            game.score_best.to_string();
    }
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<Button>),
    >,
    run_state: Res<State<RunState>>,
    mut next_state: ResMut<NextState<RunState>>,
    ui_assets: Res<UiAssets>,
) {
    for (interaction, mut image) in
        interaction_query.iter_mut()
    {
        match (interaction, run_state.get()) {
            (_, RunState::Startup) => {}
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
                *image =
                    ui_assets.button_red.clone().into();
                // tint button slightly darker
                image.color = Color::srgb(0.9, 0.9, 0.9);
            }
            (Interaction::Hovered, RunState::GameOver) => {
                *image = ui_assets.button.clone().into();
                // tint button slightly darker
                image.color = Color::srgb(0.9, 0.9, 0.9);
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
    mut text_query: Query<Entity, With<NewGameButtonText>>,
    run_state: Res<State<RunState>>,
    mut writer: TextUiWriter,
) {
    let Ok(text_entity) = text_query.get_single_mut()
    else {
        error!("Expected a single NewGameButtonText");
        return;
    };

    let new_text = match run_state.get() {
        RunState::Playing | RunState::Startup => {
            "End Game".to_string()
        }
        RunState::GameOver => "New Game".to_string(),
    };

    *writer.text(text_entity, 0) = new_text;
}
