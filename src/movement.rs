use bevy::prelude::*;

use crate::tracker::{Track, TrackerPlugin};

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Event)]
pub struct WantsToMove {
    entity: Entity,
    position: Position,
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub position: Position,
    pub velocity: Velocity,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TrackerPlugin::<Position>::default())
            .add_event::<WantsToMove>()
            .add_systems(Update, interpolate_movement)
            .add_systems(FixedUpdate, (movement_intent, apply_movement).chain());
    }
}

pub fn movement_intent(
    time: Res<Time>,
    query: Query<(Entity, &Velocity, &Position)>,
    mut event_writer: EventWriter<WantsToMove>,
) {
    for (entity, velocity, position) in query.iter() {
        let position = Position(position.0 + velocity.0 * time.delta_seconds());

        event_writer.send(WantsToMove { entity, position });
    }
}

pub fn apply_movement(mut query: Query<&mut Position>, mut event_reader: EventReader<WantsToMove>) {
    for event in event_reader.read() {
        println!("move intent");

        if let Ok(mut position) = query.get_mut(event.entity) {
            *position = event.position;
        }
    }
}

fn interpolate_movement(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Position, &Track<Position>)>,
) {
    for (mut transform, position, old_position) in query.iter_mut() {
        transform.translation = old_position
            .0
            .lerp(position.0, fixed_time.overstep_fraction())
            .extend(0.);
    }
}
