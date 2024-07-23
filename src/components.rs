use bevy::prelude::*;
use std::{borrow::Cow, time::Duration};

pub mod characteristics;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// #[derive(Component)]
// pub struct Name(Cow<'static, str>);

// impl Name {
//     pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
//         Self(name.into())
//     }
// }

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MoveSpeed(pub f32);

#[derive(Component)]
pub struct Health {
    max: u32,
    current: u32,
}

impl Health {
    pub fn new(health: u32) -> Self {
        Self {
            max: health,
            current: health,
        }
    }
}


#[derive(Component)]
pub struct Vector(pub Vec2);

