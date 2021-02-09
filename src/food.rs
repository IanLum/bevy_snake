use crate::{
    board::BoardPositions,
    components::{Position, Size},
    game::{FoodSpawnEvent, Materials},
    snake::{SnakeHead, SnakeSegment},
};
use bevy::prelude::{
    Commands, EventReader, Events, Local, Or, Query, Res, SpriteBundle, With,
};
use rand::seq::IteratorRandom;
use std::collections::HashSet;

pub struct Food;

pub fn spawn_food(
    commands: &mut Commands,
    materials: Res<Materials>,
    spawn_events: Res<Events<FoodSpawnEvent>>,
    mut spawn_reader: Local<EventReader<FoodSpawnEvent>>,
    board_positions: Res<BoardPositions>,
    snake_positions: Query<&Position, Or<(With<SnakeHead>, With<SnakeSegment>)>>,
) {
    if spawn_reader.iter(&spawn_events).next().is_some() {
        let snake_pos_hash = snake_positions.iter().copied().collect::<HashSet<_>>();
        let open_positions = board_positions.0.difference(&snake_pos_hash);
        let spawn_pos = open_positions.choose(&mut rand::thread_rng()).unwrap();

        commands
            .spawn(SpriteBundle {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .with(Food)
            .with(Position {
                x: spawn_pos.x,
                y: spawn_pos.y,
            })
            .with(Size::square(0.8));
    }
}
