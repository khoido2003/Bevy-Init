use bevy::prelude::*;

pub mod component;
pub mod resource;
pub mod system;

use resource::*;
use system::*;

use crate::AppState;

use super::SimulationState;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::default())
            .add_system(spawn_enemy.in_schedule(OnEnter(AppState::Game)))
            .add_system(
                enemy_movement
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            //.add_system(update_enemy_direction)
            //.add_system(confine_enemy_movement)
            //.add_system(tick_enemy_spawn_timer)
            //.add_system(spawn_enemy_over_time);
            .add_systems(
                (
                    update_enemy_direction,
                    confine_enemy_movement,
                    spawn_enemy_over_time,
                    tick_enemy_spawn_timer,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            );
    }
}
