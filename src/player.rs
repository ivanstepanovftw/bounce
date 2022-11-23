use bevy::prelude::*;
use crate::prelude::{AppState, Graphics};

pub struct PlayerPlugin;

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Component)]
pub struct RigidBody;

#[derive(Component)]
pub enum Collider {
    Rect(f32, f32), // width, height
    Circle(f32), // radius
}

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Acceleration(pub Vec2);

#[derive(Component)]
pub struct OnGround(pub bool);


const GRAVITY: Vec2 = Vec2::new(0., -1000.0);


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(Self::setup_player)
            .add_system(Self::player_controller)
            .add_system(Self::physics);
    }
}

impl PlayerPlugin {
    fn setup_player(
        mut commands: Commands,
        mut graphics: ResMut<Graphics>,
        asset_server: Res<AssetServer>
    ) {
        commands
            .spawn(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(2),
                texture_atlas: graphics.texture_atlas.clone(),
                transform: Transform::from_xyz(0., 0., 100.),
                ..default()
            })
            .insert(Player)
            .insert(RigidBody)
            .insert(Collider::Circle(40.))
            .insert(Position(Vec2::new(0., 80.)))
            .insert(Velocity(Vec2::ZERO))
            .insert(Acceleration(GRAVITY))
            .insert(OnGround(false));
    }

    fn player_controller(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Acceleration, &mut Velocity, &OnGround), (With<Player>, With<RigidBody>)>,
    ) {
        for (mut acceleration, mut velocity, on_ground) in query.iter_mut() {
            // Moving
            if keyboard_input.pressed(KeyCode::Left) {
                // acceleration.0.x = -100.;
                velocity.0.x = -200.;
            } else if keyboard_input.pressed(KeyCode::Right) {
                // acceleration.0.x = 100.;
                velocity.0.x = 200.;
            } else {
                velocity.0.x = 0.;
                // acceleration.0.x = 0.;
            }

            // Jump
            if keyboard_input.pressed(KeyCode::Up) && on_ground.0 {
                velocity.0.y = 400.;
            }
        }

    }

    fn physics(
        time: Res<Time>,
        mut query: Query<(&Player, &mut Transform, &mut Position, &mut Velocity, &mut Acceleration, &mut OnGround)>,
    ) {
        for (player, mut transform, mut position, mut velocity, mut acceleration, mut on_ground) in query.iter_mut() {
            // Velocity
            velocity.0 += acceleration.0 * time.delta_seconds();
            
            // Position
            position.0 += velocity.0 * time.delta_seconds();

            // Collision
            if position.0.y <= 0. {
                position.0.y = 0.;
                on_ground.0 = true;
            } else {
                on_ground.0 = false;
            }

            // Transform
            transform.translation = position.0.extend(0.);
            transform.translation.z = 100.; // FIXME: keep Z index
        }
    }
}
