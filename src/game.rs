// mod fire_ball;
mod lifetime;
pub mod player;
mod characteristics;

use bevy::prelude::*;

use self::{lifetime::LifetimePlugin, player::PlayerPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, LifetimePlugin))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// fn player_attack(
//     mut query: Query<(&ActionState<Action>, &Transform, &mut AttackSpeed), With<Player>>,
//     time: Res<Time>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let (action_state, player_transform, mut attack_speed) = query.single_mut();

//     attack_speed.tick(time.delta());

//     if action_state.pressed(&Action::Attack) {
//         if let Some(_can_attack) = attack_speed.try_trigger() {
//             commands.spawn((
//                 MovementBundle {
//                     position: Position(player_transform.translation.xy()),
//                     velocity: Velocity((player_transform.rotation * Vec3::Y).xy() * 50.),
//                 },
//                 MaterialMesh2dBundle {
//                     mesh: meshes.add(Circle::default()).into(),
//                     transform: Transform::default().with_scale(Vec3::splat(12.)),
//                     material: materials.add(Color::from(BLUE)),
//                     ..default()
//                 },
//             ));
//         }
//     }
// }
