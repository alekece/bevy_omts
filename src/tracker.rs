use std::marker::PhantomData;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(Component, Deref, DerefMut)]
pub struct Track<T>(T);

pub struct TrackerPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for TrackerPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<T> Plugin for TrackerPlugin<T>
where
    T: 'static + Send + Sync + Component + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, track_value::<T>);
    }
}

fn track_value<T: Component + Clone>(query: Query<(Entity, &T)>, mut commands: Commands) {
    for (e, value) in query.iter() {
        commands.entity(e).insert(Track(value.clone()));
    }
}
