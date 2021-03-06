#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}
#[derive(Debug)]
pub struct Block {
    pub value: u32,
}
pub struct BlockText;

pub struct Board {
    pub size: u8,
}

#[derive(Default)]
pub struct Game {
    pub score: u32,
    pub score_best: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RunState {
    Playing,
    GameOver,
}
