use bevy::prelude::*;

pub mod component;
pub mod resource;
pub mod system;

use resource::*;
use system::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .insert_resource(Score::default())
            .add_system(update_score)
            .add_system(update_high_score)
            .add_system(high_score_updated);
    }
}
