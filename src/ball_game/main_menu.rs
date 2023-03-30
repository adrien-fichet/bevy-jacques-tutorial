pub mod components;
pub mod interactions;
pub mod layout;
pub mod styles;

pub use components::*;
pub use interactions::*;
pub use layout::*;
pub use styles::*;

use crate::ball_game::*;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_systems((
                interact_with_play_button,
                interact_with_quit_button
            ).in_set(OnUpdate(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
