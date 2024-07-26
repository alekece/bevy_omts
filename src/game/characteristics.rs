use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Component)]
pub struct AttackSpeed {
    pub attacks_per_second: f32,
}

#[derive(Component)]
pub struct MoveSpeed {
    pub meters_per_second: f32,
}

#[derive(Component)]
pub struct Luck {
    pub critical_hit_percentage: f32,
    pub dogde_percentage: f32,
}
