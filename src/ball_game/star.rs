use bevy::prelude::*;
use crate::ball_game::*;

pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            .add_systems((
                spawn_stars_over_time,
                tick_star_spawn_timer,
                player_hit_star,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)))
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
