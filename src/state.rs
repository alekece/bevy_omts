use crate::prelude::*;

#[derive(States, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Setup,
    InGame {
        paused: bool,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct InGame;

impl ComputedStates for InGame {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::InGame { .. } => Some(InGame),
            _ => None,
        }
    }
}

#[derive(SubStates, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[source(InGame=InGame)]
pub enum GamePhase {
    #[default]
    Setup,
    Battle,
    LevelUp,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_computed_state::<InGame>()
            .add_sub_state::<GamePhase>()
            .enable_state_scoped_entities::<GameState>()
            .add_systems(Update, transition_to_game.run_if(in_state(GameState::Setup)));
    }
}

fn transition_to_game(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::InGame { paused: false })
}
