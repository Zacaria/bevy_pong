use bevy::{
    asset::ChangeWatcher,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::collider::Collider;
use crate::movement::Velocity;

const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.);
const BALL_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct Ball;

pub fn spawn_ball(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
        // Collider,
        Name::new("Ball"),
    ));
}
