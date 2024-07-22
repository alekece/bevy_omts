use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Actionlike, Reflect)]
pub enum Action {
    Up,
    Down,
    Right,
    Left,
    Ability,
    Attack,
}
