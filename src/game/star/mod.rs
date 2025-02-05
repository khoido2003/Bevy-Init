use bevy::prelude::*;

pub mod component;
pub mod resource;
pub mod system;

use super::SimulationState;
use crate::AppState;
use resource::*;
use system::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StarSpawnTimer::default())
            .add_system(spawn_star.in_schedule(OnEnter(AppState::Game)))
            //.add_system(tick_star_spawn_timer)
            //.add_system(spawn_stars_over_time)
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
