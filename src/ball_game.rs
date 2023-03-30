mod constants;
mod enemy;
mod events;
mod player;
mod score;
mod star;
mod systems;
mod main_menu;

pub use constants::*;
pub use enemy::*;
pub use events::*;
pub use player::*;
pub use score::*;
pub use star::*;
pub use systems::*;
pub use main_menu::*;

use bevy::prelude::*;


struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(ScorePlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        //.add_system(bevy::window::close_on_esc)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
