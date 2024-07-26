use crate::prelude::*;

#[derive(Resource, Default)]
pub struct Cursor {
    pub position: Vec2,
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_cursor);
    }
}

fn update_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window_query: Query<&Window>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    if let Some(position) = window
        .cursor_position()
        .and_then(|position| camera.viewport_to_world_2d(camera_transform, position))
    {
        commands.insert_resource(Cursor { position });
    }
}
