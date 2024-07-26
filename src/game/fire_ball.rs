use bevy::prelude::*;

use crate::physics::{AutoMovementPlugin, LinearVelocity, Position};

use super::lifetime::Lifetime;

#[derive(Component)]
pub struct FireBall;

#[derive(Bundle)]
pub struct FireBallBundle {
    position: Position,
    velocity: LinearVelocity,
    lifetime: Lifetime,
}

pub struct FireBallPlugin;

impl Plugin for FireBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AutoMovementPlugin::<FireBall>::default());
    }
}
