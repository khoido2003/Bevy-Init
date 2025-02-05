use bevy::prelude::*;

pub const STAR_SPAWN_TIMER: f32 = 1.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}
