use bevy::prelude::*;

pub mod component;
pub mod resource;
pub mod system;

use resource::*;
use system::*;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StarSpawnTimer::default())
            .add_startup_system(spawn_star)
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_stars_over_time);
    }
}
