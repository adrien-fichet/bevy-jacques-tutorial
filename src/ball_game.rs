mod events;
mod constants;
mod enemy;
mod player;
mod score;
mod star;
mod systems;

pub use events::*;
pub use player::*;
pub use enemy::*;
pub use score::*;
pub use star::*;
pub use constants::*;
pub use systems::*;

use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScorePlugin)
        //.add_system(bevy::window::close_on_esc)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}
