use rustyhack_lib::ecs::components::{Character, EntityColour, Position, EntityName};

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub entity_name: EntityName,
    pub position: Position,
    pub character: Character,
    pub entity_colour: EntityColour,
}
