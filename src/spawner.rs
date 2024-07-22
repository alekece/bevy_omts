use bevy::prelude::*;

use crate::components::{Player, Position};

pub fn spawn_player(commands: &mut Commands) {
    commands.spawn((Player));
}
