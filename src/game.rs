use crate::{
    components::*,
    food::Food,
    snake::{spawn_segment, SnakeHead, SnakeSegment, SnakeSegments},
};
use bevy::prelude::{
    Assets, Camera2dBundle, Color, ColorMaterial, Commands, Entity, EventReader, Events, Handle,
    Local, Query, Res, ResMut, SpriteBundle, With,
};

pub struct GrowthEvent;

pub struct FoodSpawnEvent;

pub struct GameOverEvent;

pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub segment_material: Handle<ColorMaterial>,
    pub food_material: Handle<ColorMaterial>,
}

pub fn initial_spawn(
    commands: &mut Commands,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
    mut spawn_events: ResMut<Events<FoodSpawnEvent>>,
) {
    segments.0 = vec![
        commands
            .spawn(SpriteBundle {
                material: materials.head_material.clone(),
                ..Default::default()
            })
            .with(SnakeHead {
                direction: Direction::Up,
            })
            .with(SnakeSegment)
            .with(Position { x: 3, y: 3 })
            .with(Size::square(0.8))
            .current_entity()
            .unwrap(),
        spawn_segment(
            commands,
            &materials.segment_material,
            Position { x: 3, y: 2 },
        ),
    ];

    spawn_events.send(FoodSpawnEvent);
}

pub fn game_over(
    commands: &mut Commands,
    mut reader: Local<EventReader<GameOverEvent>>,
    game_over_events: Res<Events<GameOverEvent>>,
    materials: Res<Materials>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    spawn_events: ResMut<Events<FoodSpawnEvent>>,
) {
    if reader.iter(&game_over_events).next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.despawn(ent);
        }
        initial_spawn(commands, materials, segments_res, spawn_events)
    }
}

pub fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
}
