use bevy::{
    color::palettes::tailwind::*, math::U16Vec2, prelude::*,
};
use itertools::Itertools;
use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(
            Color::from(Srgba::hex("#1f2638")
                .expect("developer should have provided a valid hex code"))
        ))
        .insert_resource(Board::new(4))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2048".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_board, spawn_tiles))
        .add_systems(Update, sync_tile_points)
        .run();
}

#[derive(Resource, Clone)]
struct Board {
    size: u16,
    world_size: f32,
    tile_size: f32,
    tile_spacer: f32,
}

impl Board {
    fn new(size: u16) -> Self {
        let tile_size: f32 = 40.0;
        let tile_spacer: f32 = 10.0;

        let world_size = f32::from(size) * tile_size
            + f32::from(size + 1) * tile_spacer;
        Board {
            size,
            world_size,
            tile_size,
            tile_spacer,
        }
    }
    fn grid_to_world_position(&self, pos: u16) -> f32 {
        let offset =
            -self.world_size / 2.0 + 0.5 * self.tile_size;

        offset
            + f32::from(pos) * self.tile_size
            + f32::from(pos + 1) * self.tile_spacer
    }
}

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
struct Position(U16Vec2);

#[derive(Component)]
struct TileText;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_board(mut commands: Commands, board: Res<Board>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::from(SLATE_900),
                custom_size: Some(Vec2::splat(
                    board.world_size,
                )),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for tile in (0..board.size)
                .cartesian_product(0..board.size)
            {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(
                            0.54, 0.64, 0.72,
                        ),
                        custom_size: Some(Vec2::splat(
                            board.tile_size,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board
                            .grid_to_world_position(tile.0),
                        board
                            .grid_to_world_position(tile.1),
                        1.0,
                    ),
                    ..default()
                });
            }
        });
}

fn spawn_tiles(mut commands: Commands, board: Res<Board>) {
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<(u16, u16)> = (0..board.size)
        .cartesian_product(0..board.size)
        .choose_multiple(&mut rng, 2);

    for (x, y) in starting_tiles.into_iter() {
        let pos = Position(U16Vec2::new(x, y));
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(
                            0.84, 0.89, 0.93,
                        ),
                        custom_size: Some(Vec2::splat(
                            board.tile_size,
                        )),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board.grid_to_world_position(
                            pos.0.x,
                        ),
                        board.grid_to_world_position(
                            pos.0.y,
                        ),
                        1.0,
                    ),
                    ..default()
                },
                Points { value: 2 },
                pos,
            ))
            .with_children(|child_builder| {
                child_builder.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "2",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_xyz(
                            0.0, 0.0, 1.0,
                        ),
                        ..default()
                    },
                    TileText,
                ));
            });
    }
}

fn sync_tile_points(
    mut texts: Query<
        (&mut Text, &mut Transform, &Parent),
        With<TileText>,
    >,
    tiles: Query<&Points>,
) {
    for (mut text, mut transform, parent) in &mut texts {
        let Ok(points) = tiles.get(parent.get()) else {
            warn!("An entity with TileText should have a parent with a Points component");
            continue;
        };

        text.sections[0].value = points.value.to_string();
        *transform = transform.with_scale(Vec3::splat(
            1.0 / points.value.to_string().len() as f32,
        ));
    }
}
