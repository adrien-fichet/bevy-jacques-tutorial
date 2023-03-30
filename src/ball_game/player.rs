use bevy::prelude::*;

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

use crate::ball_game::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems((
                player_movement.in_set(MovementSystemSet),
                confine_player_movement.in_set(ConfinementSystemSet)
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
