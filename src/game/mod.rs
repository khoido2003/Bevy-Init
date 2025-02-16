use bevy::{
    app::IntoSystemAppConfig,
    prelude::{in_state, IntoSystemConfig, OnEnter, OnExit, Plugin, States},
};
use system::{pause_simualation, resume_simualation, toggle_simualation};

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod system;

use crate::{events::*, AppState};
use enemy::{system::despawn_enemies, EnemyPlugin};
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_system(pause_simualation.in_schedule(OnEnter(AppState::Game)))
            .add_plugin(EnemyPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(StarPlugin)
            .add_system(toggle_simualation.run_if(in_state(AppState::Game)))
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)))
            .add_system(resume_simualation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
