use bevy::prelude::*;

pub mod component;
pub mod resource;
pub mod system;

use resource::*;
use system::*;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer::default())
            .add_startup_system(spawn_enemy)
            .add_system(enemy_movement)
            .add_system(update_enemy_direction)
            .add_system(confine_enemy_movement)
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_enemy_over_time);
    }
}
