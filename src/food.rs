
use bevy::prelude::*;
use crate::grid::{Position, Size};
use crate::grid;
use std::time::Duration;
use super::Materials;
use rand::prelude::random;

pub struct Food;

pub struct FoodSpawnTimer(Timer);
impl Default for FoodSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(2000), true))
    }
}

pub fn food_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: Local<FoodSpawnTimer>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        commands
            .spawn(SpriteComponents {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .with(Food)
            .with(Position {
                x: (random::<f32>() * grid::ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * grid::ARENA_HEIGHT as f32) as i32,
            })
            .with(Size::square(0.8));
    }
}