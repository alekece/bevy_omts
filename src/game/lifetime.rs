use bevy::prelude::*;

#[derive(Component)]
pub struct Lifetime(pub Timer);

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, remove_stale_entities);
    }
}

fn remove_stale_entities(mut query: Query<(Entity, &mut Lifetime)>, mut commands: Commands, time: Res<Time>) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.0.tick(time.delta());

        if lifetime.0.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
