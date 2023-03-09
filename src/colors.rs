use bevy::prelude::Color;

pub struct Materials {
    pub board: Color,
    pub tile_placeholder: Color,
    pub tile: Color,
    pub none: Color,
}
pub const MATERIALS: Materials = Materials {
    board: Color::Lcha {
        lightness: 0.15,
        chroma: 0.5,
        hue: 315.0,
        alpha: 1.0,
    },
    tile_placeholder: Color::Lcha {
        lightness: 0.55,
        chroma: 0.5,
        hue: 315.0,
        alpha: 1.0,
    },
    tile: Color::Lcha {
        lightness: 0.75,
        chroma: 0.5,
        hue: 315.0,
        alpha: 1.0,
    },
    none: Color::NONE,
};

pub struct ButtonMaterials {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

pub const BUTTON_MATERIALS: ButtonMaterials =
    ButtonMaterials {
        normal: Color::Lcha {
            lightness: 0.15,
            chroma: 0.5,
            hue: 315.0,
            alpha: 1.0,
        },
        hovered: Color::Lcha {
            lightness: 0.55,
            chroma: 0.5,
            hue: 315.0,
            alpha: 1.0,
        },
        pressed: Color::Lcha {
            lightness: 0.75,
            chroma: 0.5,
            hue: 315.0,
            alpha: 1.0,
        },
    };
