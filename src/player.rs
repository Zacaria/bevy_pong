use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::collider::Collider;
use crate::constants::*;

// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;
const PADDLE_SPEED: f32 = 900.0;
const PADDLE_SIZE: Vec3 = Vec3::new(20.0, 120.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.8, 0.4, 0.4);
const PADDLE_INIT_POSITION: f32 = 300.0;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Up,
    Down,
}

#[derive(Component, Debug, Copy, Clone)]
pub enum PlayerSide {
    Left,
    Right,
}

impl PlayerSide {
    fn movement_keys(&self) -> (KeyCode, KeyCode) {
        match self {
            PlayerSide::Left => (KeyCode::W, KeyCode::S),
            PlayerSide::Right => (KeyCode::Up, KeyCode::Down),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    name: Name,
    side: PlayerSide,
    input_manager: InputManagerBundle<Action>,
}

impl PlayerBundle {
    fn new(side: PlayerSide, init_position: f32, name: String) -> Self {
        PlayerBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(init_position, 0.0, 0.0),
                    scale: PADDLE_SIZE,
                    // rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                    ..default()
                },
                sprite: Sprite {
                    color: PADDLE_COLOR,
                    ..default()
                },
                ..default()
            },
            side,
            collider: Collider,
            name: Name::new(name),
            input_manager: InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (side.movement_keys().0, Action::Up),
                    (side.movement_keys().1, Action::Down),
                ]),
            },
        }
    }
}

pub fn spawn_paddles(mut commands: Commands) {
    commands.spawn(PlayerBundle::new(
        PlayerSide::Left,
        -PADDLE_INIT_POSITION,
        "Left Paddle".to_string(),
    ));
    commands.spawn(PlayerBundle::new(
        PlayerSide::Right,
        PADDLE_INIT_POSITION,
        "Right Paddle".to_string(),
    ));
}

pub fn paddle_movement(
    mut query_player: Query<(&mut Transform, &ActionState<Action>)>,
    time_step: Res<FixedTime>,
) {
    for (mut player_transform, action_state) in query_player.iter_mut() {
        let mut direction = 0.0;
        if action_state.pressed(Action::Up) {
            direction += 1.0;
        }

        if action_state.pressed(Action::Down) {
            direction -= 1.0;
        }

        let new_position = player_transform.translation.y
            + direction * PADDLE_SPEED * time_step.period.as_secs_f32();

        let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;
        let bottom_bound =
            BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;

        player_transform.translation.y = new_position.clamp(bottom_bound, top_bound);
    }
}
