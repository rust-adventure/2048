use bevy::prelude::Color;

pub const BOARD: Color = Color::Lcha {
    lightness: 0.06,
    chroma: 0.088,
    hue: 281.0,
    alpha: 1.0,
};

pub const TILE_PLACEHOLDER: Color = Color::Lcha {
    lightness: 0.55,
    chroma: 0.5,
    hue: 315.0,
    alpha: 1.0,
};

pub const TILE: Color = Color::Lcha {
    lightness: 0.85,
    chroma: 0.5,
    hue: 315.0,
    alpha: 1.0,
};

pub const SCORE_BOX: Color = Color::Lcha {
    lightness: 0.55,
    chroma: 0.5,
    hue: 315.0,
    alpha: 1.0,
};
