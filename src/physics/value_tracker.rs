use std::marker::PhantomData;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(Component, Deref, DerefMut)]
pub struct Track<T>(T);

pub struct ValueTrackerPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T> Default for ValueTrackerPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
        }
    }
}

impl<T> Plugin for ValueTrackerPlugin<T>
where
    T: 'static + Send + Sync + Component + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, track_value::<T>);
    }
}

fn track_value<T: Component + Clone>(query: Query<(Entity, &T)>, mut commands: Commands) {
    for (e, value) in query.iter() {
        let _ = commands.entity(e).try_insert(Track(value.clone()));
    }
}
