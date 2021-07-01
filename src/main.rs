use bevy::prelude::*;

const TILE_SIZE: f32 = 40.0;

struct Board {
    size: u8,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_board.system())
        .run()
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_board(mut commands: Commands) {
    let board = Board { size: 4 };
    let physical_board_size =
        f32::from(board.size) * TILE_SIZE;

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(
                physical_board_size,
                physical_board_size,
            )),
            ..Default::default()
        })
        .insert(board);
}
