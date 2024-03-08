use bevy::prelude::Color;

pub mod palette {
    use super::*;

    pub const TILE_PLACEHOLDER: Color =
        Color::rgb(0.54, 0.64, 0.72);
    pub const TILE: Color = Color::rgb(0.63, 0.74, 0.83);
    pub const NONE: Color = Color::NONE;
}
