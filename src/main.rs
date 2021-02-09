mod board;
mod components;
mod food;
mod game;
mod snake;

use crate::{board::*, food::*, game::*, snake::*};
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::time::Duration;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "snake".to_string(),
            width: 700.0,
            height: 700.0,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(SnakeMoveTimer(Timer::new(
            Duration::from_millis(150. as u64), //default 150
            true,
        )))
        .add_resource(SnakeSegments::default())
        .add_resource(LastTailPosition::default())
        .add_resource(BoardPositions::default())
        .add_event::<GrowthEvent>()
        .add_event::<FoodSpawnEvent>()
        .add_event::<GameOverEvent>()
        .add_startup_system(setup.system())
        .add_startup_stage("spawn", SystemStage::single(initial_spawn.system()))
        .add_system(snake_movement.system())
        .add_system(snake_timer.system())
        .add_system(snake_eating.system())
        .add_system(snake_growth.system())
        .add_system(position_translation.system())
        .add_system(size_scaling.system())
        .add_system(spawn_food.system())
        .add_system(game_over.system())
        .add_plugins(DefaultPlugins)
        .run();
}
