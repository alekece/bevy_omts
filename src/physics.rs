mod movement;
mod value_tracker;

use bevy::prelude::*;

pub use self::movement::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MovementPlugin);
    }
}
