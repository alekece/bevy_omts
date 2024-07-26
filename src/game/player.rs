use bevy::{color::palettes::css::RED, sprite::MaterialMesh2dBundle};

use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Actionlike, Reflect)]
pub enum PlayerAction {
    MoveForward,
    MoveAside,
    Attack,
    CastAbility,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(OnEnter(GamePhase::Setup), setup_player);
        // .add_systems(
        //     Update,
        //     (
        //         move_player,
        //         (rotate_player, player_attack).run_if(resource_exists::<Cursor>),
        //     ),
        // );
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let input_map = InputMap::default()
        .insert_one_to_many(
            PlayerAction::MoveForward,
            [VirtualAxis::ws(), VirtualAxis::vertical_arrow_keys()],
        )
        .insert_one_to_many(
            PlayerAction::MoveAside,
            [VirtualAxis::ad(), VirtualAxis::horizontal_arrow_keys()],
        )
        .insert(PlayerAction::Attack, MouseButton::Left)
        .build();

    commands.spawn((
        Player,
        StateScoped(InGame),
        RigidBody::Kinematic,
        InputManagerBundle::with_map(input_map),
        MaterialMesh2dBundle {
            mesh: meshes.add(Triangle2d::default()).into(),
            transform: Transform::default().with_scale(Vec3::splat(64.)),
            material: materials.add(Color::from(RED)),
            ..default()
        },
    ));
}

fn move_player(
    mut query: Query<(Entity, &ActionState<PlayerAction>, &mut LinearVelocity), With<Player>>,
    // mut commands: Commands,
) {
    let (entity, action_state, mut velocity) = query.single_mut();

    let direction = Vec2::new(
        action_state.value(&PlayerAction::MoveAside),
        action_state.value(&PlayerAction::MoveForward),
    );

    velocity.0 = direction.normalize_or_zero() * 50.;

    // commands.entity(entity).insert(WantsToMove(position.0 + velocity));
}

// fn rotate_player(mut player_query: Query<(&mut Transform, &MoveSpeed), With<Player>>, cursor: Res<Cursor>, time: Res<Time>) {
//     let (mut player_transform, move_speed) = player_query.single_mut();

//     let cursor_direction = (cursor.position - player_transform.translation.xy()).normalize();
//     let rotation_to_cursor = Quat::from_rotation_arc(Vec3::Y, cursor_direction.extend(0.));

//     player_transform.rotation = player_transform
//         .rotation
//         .lerp(rotation_to_cursor, time.delta_seconds() * move_speed.meters_per_second);
// }

// fn player_attack(
//     player_query: Query<(&ActionState<PlayerAction>, &Transform), With<Player>>,
//     cursor: Res<Cursor>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut commands: Commands,
// ) {
//     let (action_state, player_transform) = player_query.single();

//     if action_state.pressed(&PlayerAction::Attack) {
//         let direction = (cursor.position - player_transform.translation.xy()).normalize();
//         let position = player_transform.translation.xy() + direction * 50.;
//         let velocity = direction * 500.;

//         commands.spawn((
//             FireBall,
//             Position(position),
//             LinearVelocity(velocity),
//             Lifetime(Timer::from_seconds(1., TimerMode::Once)),
//             MaterialMesh2dBundle {
//                 mesh: meshes.add(Circle::default()).into(),
//                 transform: Transform::default()
//                     .with_translation(position.extend(0.))
//                     .with_scale(Vec3::splat(12.)),
//                 material: materials.add(Color::from(BLUE)),
//                 ..default()
//             },
//         ));
//     }
// }
