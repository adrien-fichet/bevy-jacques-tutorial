use bevy::prelude::*;

pub mod resources;
pub mod systems;

pub use resources::*;
pub use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_system(update_high_scores)
            .add_system(high_scores_updated)
            .add_system(update_score);
    }
}
