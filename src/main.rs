use std::time::Duration;

use bevy::{
    asset::ChangeWatcher,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use bevy_editor_pls::EditorPlugin;
use bevy_pong::collider::Collider;
use bevy_pong::player::{paddle_movement, spawn_paddles};
use bevy_pong::wall::spawn_walls;
use bevy_pong::{
    ball::{spawn_ball, Ball},
    movement::apply_velocity,
};
use bevy_pong::{movement::Velocity, player::PlayerSide};

fn main() {
    App::new()
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ball)
        .add_systems(Startup, spawn_paddles)
        .add_systems(Startup, spawn_walls)
        .add_systems(
            Update,
            (
                check_collisions,
                apply_velocity.before(check_collisions),
                paddle_movement
                    .before(check_collisions)
                    .after(apply_velocity),
            ),
        )
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            ..Default::default()
        }))
        .add_plugins(EditorPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(UiCameraBundle::default());
}

fn check_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform, &Sprite), With<Ball>>,
    collider_query: Query<(&Transform), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform, ball_sprite) = ball_query.single_mut();

    for (transform) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_sprite.custom_size.unwrap(),
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            if collision == Collision::Left || collision == Collision::Right {
                dbg!(&ball_velocity, &reflect_x, &reflect_y);
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}
