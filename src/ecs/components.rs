use console_engine::Color;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpenState {
    pub open: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CollisionState {
    pub collidable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Character {
    pub icon: char,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityColour {
    pub colour: Color,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsPlayer {
    pub is_player: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VisibleState {
    pub visible: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerId {
    pub uuid: Uuid,
}
