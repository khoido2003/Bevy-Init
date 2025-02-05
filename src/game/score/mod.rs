use bevy::prelude::*;

pub mod component;
pub mod resource;
pub mod system;

use resource::*;
use system::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .insert_resource(Score::default())
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_system(update_high_score)
            .add_system(high_score_updated);
    }
}
