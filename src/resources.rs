use bevy::prelude::*;

const ASTEROID_SPAWN_DELAY_SECS: f32 = 1.0;

#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> AsteroidSpawnTimer {
        AsteroidSpawnTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWN_DELAY_SECS, TimerMode::Repeating),
        }
    }
}
