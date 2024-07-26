// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::pedantic)]

mod cursor;
mod game;
mod state;

mod prelude {
    pub use avian2d::prelude::*;
    pub use bevy::prelude::*;
    pub use leafwing_input_manager::prelude::*;

    pub use crate::cursor::*;
    pub use crate::game::*;
    pub use crate::state::*;
}

use bevy::asset::AssetMetaCheck;

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics in web builds on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        meta_check: AssetMetaCheck::Never,
        ..default()
    }));

    #[cfg(debug_assertions)]
    app.add_plugins(PhysicsDebugPlugin::default());

    app.add_plugins((PhysicsPlugins::default(), GamePlugin, CursorPlugin, StatePlugin))
        .insert_resource(Gravity::ZERO);

    app.run();
}
