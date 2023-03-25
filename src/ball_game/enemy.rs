use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

pub use components::*;
pub use resources::*;
pub use systems::*;

use crate::ball_game::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            //.add_startup_system(spawn_enemies)
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_systems((
                spawn_enemies_on_e,
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                tick_enemy_spawn_timer,
                enemy_hit_player,
                spawn_enemy_over_time,
                rotate_enemies,)
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running)
        ))
        .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }    
}
