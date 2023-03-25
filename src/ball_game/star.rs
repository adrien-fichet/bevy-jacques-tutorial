use bevy::prelude::*;

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
            .add_startup_system(spawn_stars)
            .add_system(spawn_stars_over_time)
            .add_system(tick_star_spawn_timer);
    }
}
