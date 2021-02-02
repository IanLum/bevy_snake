use crate::{
    board::{ARENA_HEIGHT, ARENA_WIDTH},
    components::{Position, Size},
    game::{FoodSpawnEvent, Materials},
};
use bevy::prelude::{Commands, EventReader, Events, Local, Res, SpriteBundle};
use rand::prelude::random;

pub struct Food;

pub fn spawn_food(
    commands: &mut Commands,
    materials: Res<Materials>,
    spawn_events: Res<Events<FoodSpawnEvent>>,
    mut spawn_reader: Local<EventReader<FoodSpawnEvent>>,
) {
    if spawn_reader.iter(&spawn_events).next().is_some() {
        commands
            .spawn(SpriteBundle {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .with(Food)
            .with(Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
            .with(Size::square(0.8));
    }
}
