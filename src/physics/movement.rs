use std::marker::PhantomData;

use bevy::prelude::*;

use super::value_tracker::{Track, ValueTrackerPlugin};

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct LinearVelocity(pub Vec2);

#[derive(Component)]
pub struct WantsToMove(pub Vec2);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ValueTrackerPlugin::<Position>::default())
            .add_systems(Update, interpolate_position)
            .add_systems(FixedUpdate, move_entities);
    }
}

pub fn move_entities(mut query: Query<(Entity, &mut Position, &WantsToMove)>, mut commands: Commands) {
    for (entity, mut position, WantsToMove(new_position)) in query.iter_mut() {
        position.0 = *new_position;

        commands.entity(entity).remove::<WantsToMove>();
    }
}

fn interpolate_position(fixed_time: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &Position, &Track<Position>)>) {
    for (mut transform, position, old_position) in query.iter_mut() {
        transform.translation = old_position
            .0
            .lerp(position.0, fixed_time.overstep_fraction())
            .extend(0.);
    }
}

pub struct AutoMovementPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for AutoMovementPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<T> Plugin for AutoMovementPlugin<T>
where
    T: 'static + Send + Sync + Component,
{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, intent_to_move_entities::<T>);
    }
}

pub fn intent_to_move_entities<T: Component>(
    time: Res<Time>,
    query: Query<(Entity, &LinearVelocity, &Position), With<T>>,
    mut commands: Commands,
) {
    for (entity, velocity, position) in query.iter() {
        let new_position = position.0 + velocity.0 * time.delta_seconds();

        commands.entity(entity).insert(WantsToMove(new_position));
    }
}
