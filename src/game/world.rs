use crate::prelude::*;

#[derive(PhysicsLayer)]
pub enum WorldLayer {
    Player,
    PlayerAttack,
    Enemy,
    EnemyAttack,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Setup), setup_world);
    }
}

fn setup_world(mut commands: Commands) {
    commands.insert_resource(Gravity::ZERO);
}
