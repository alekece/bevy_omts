// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::pedantic)]

use std::time::Duration;

use bevy::{
    color::palettes::basic::{BLUE, RED},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use leafwing_input_manager::prelude::*;

pub mod action;
pub mod components;
pub mod cycle;
pub mod spawner;

use action::Action;
use components::{characteristics::AttackSpeed, Health, MoveSpeed, Player, Vector};
use cycle::CycleTimer;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default())
            .insert_resource(CycleTimer(Timer::from_seconds(10.0, TimerMode::Repeating)))
            .add_systems(Startup, setup_game)
            .add_systems(
                Update,
                (move_player, rotate_player, move_entity, player_attack),
            );
    }
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut input_map = InputMap::new([
        (Action::Right, KeyCode::KeyD),
        (Action::Left, KeyCode::KeyA),
        (Action::Down, KeyCode::KeyS),
        (Action::Up, KeyCode::KeyW),
        (Action::Right, KeyCode::ArrowRight),
        (Action::Left, KeyCode::ArrowLeft),
        (Action::Down, KeyCode::ArrowDown),
        (Action::Up, KeyCode::ArrowUp),
    ]);

    input_map.insert(Action::Attack, MouseButton::Left);

    commands.spawn((
        Player,
        AttackSpeed::new(2.),
        MoveSpeed(100.),
        Health::new(10),
        InputManagerBundle::with_map(input_map),
        MaterialMesh2dBundle {
            mesh: meshes.add(Triangle2d::default()).into(),
            transform: Transform::default().with_scale(Vec3::splat(64.)),
            material: materials.add(Color::from(RED)),
            ..default()
        },
    ));
}

fn move_entity(mut query: Query<(&Vector, &mut Transform)>, time: Res<Time>) {
    for (vector, mut transform) in query.iter_mut() {
        transform.translation += vector.0.extend(0.) * time.delta_seconds();
    }
}

fn move_player(
    mut query: Query<(&ActionState<Action>, &MoveSpeed, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (action_state, move_speed, mut player_transform) = query.single_mut();
    let mut direction = Vec2::ZERO;

    if action_state.pressed(&Action::Up) {
        direction.y += 1.;
    }

    if action_state.pressed(&Action::Down) {
        direction.y -= 1.;
    }

    if action_state.pressed(&Action::Right) {
        direction.x += 1.;
    }

    if action_state.pressed(&Action::Left) {
        direction.x -= 1.;
    }

    let translation_delta = direction.normalize_or_zero() * move_speed.0 * time.delta_seconds();
    player_transform.translation += translation_delta.extend(0.);
}

fn player_attack(
    mut query: Query<(&ActionState<Action>, &Transform, &mut AttackSpeed), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (action_state, player_transform, mut attack_speed) = query.single_mut();

    attack_speed.tick(time.delta());

    if action_state.pressed(&Action::Attack) {
        if let Some(_can_attack) = attack_speed.try_trigger() {
            let vector = (player_transform.rotation * Vec3::Y).xy() * 1000.;

            commands.spawn((
                Vector(vector),
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default()).into(),
                    transform: Transform::default()
                        .with_translation(player_transform.translation)
                        .with_scale(Vec3::splat(12.)),
                    material: materials.add(Color::from(BLUE)),
                    ..default()
                },
            ));
        }
    }
}

fn rotate_player(
    query: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let (camera, camera_transform) = query.single();
    let window = window.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some(cursor_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };

    let mut player_transform = player.single_mut();

    let cursor_direction = (cursor_position - player_transform.translation.xy()).normalize();
    let rotation_to_cursor = Quat::from_rotation_arc(Vec3::Y, cursor_direction.extend(0.));

    player_transform.rotation = player_transform
        .rotation
        .lerp(rotation_to_cursor, time.delta_seconds() * 10.);
}
