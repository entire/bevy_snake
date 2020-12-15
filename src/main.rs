use bevy::prelude::*;
use crate::grid::{Position, Size};

mod grid;

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup")
        .add_startup_system_to_stage("game_setup", spawn_snake.system())
        .add_system(snake_movement.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_plugins(DefaultPlugins)
        .run();
}

// setup snake

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

// setup 2D camera
// bevy expects commands -> resources -> components/queries
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.6, 0.6, 0.6).into()),
    });
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteComponents {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(12.0, 12.0)),
            ..Default::default()
        })
        .with(SnakeHead)
        .with(Position {x:3, y:3})
        .with(Size::square(0.8));
}

// this is a Query type
fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<With<SnakeHead, &mut Transform>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            *transform.translation.x_mut() -=2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            *transform.translation.x_mut() +=2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            *transform.translation.y_mut() -=2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            *transform.translation.y_mut() +=2.;
        }
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / grid::ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / grid::ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, grid::ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, grid::ARENA_HEIGHT as f32),
            0.0,
        );
    }
}