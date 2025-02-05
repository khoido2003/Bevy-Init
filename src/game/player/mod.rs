use bevy::prelude::*;

pub mod component;
pub mod system;

use system::*;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(MovementSystemSet.before(ConfinementSystem))
            //.add_startup_system(spawn_player)
            //.add_system(player_movement.before(confine_player_movement))
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(
                player_movement
                    .in_set(MovementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_system(
                confine_player_movement
                    .in_set(ConfinementSystem)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_system(confine_player_movement)
            //.add_system(enemy_hit_player)
            //.add_system(player_hit_star)
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
