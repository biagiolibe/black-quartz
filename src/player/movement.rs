use crate::map::TILE_SIZE;
use crate::player::components::*;
use crate::prelude::{DrillAnimation, GameAssets, LoadingProgress};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, Damping, GravityScale, LockedAxes, QueryFilter, ReadRapierContext,
    RigidBody, ShapeCastOptions, Velocity,
};
use bevy_rapier2d::rapier::prelude::SharedShape;

pub fn spawn_player(
    player: Query<Entity, With<Player>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut loading_progress: ResMut<LoadingProgress>,
) {
    if let Ok(entity) = player.single() {
        commands.entity(entity).despawn();
    }
    info!("Spawning Drilling Machine (Player)");
    commands
        .spawn((
            Player,
            Health::default(),
            Fuel::default(),
            Inventory::default(),
            Currency::default(),
            FieldOfView::default(),
            DrillState::default(),
            PlayerAttributes::default(),
            PlayerDirection::default(),
            DrillAnimation::default(),
        ))
        .insert((
            Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            },
            Sprite {
                image: game_assets.player.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: game_assets.player.texture_layout.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, 25.0, 0.0),
            RigidBody::Dynamic,
            Collider::capsule_y((TILE_SIZE - 28.0) / 2f32, 14.0),
            ActiveEvents::COLLISION_EVENTS,
            GravityScale(1.0),
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED,
        ));
    loading_progress.spawning_player = true;
}

pub fn move_player(
    time: Res<Time<Fixed>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query_player: Query<
        (&mut Velocity, &mut DrillState, &PlayerAttributes, &mut Fuel),
        With<Player>,
    >,
) {
    if let Ok((mut velocity, mut drill_state, attributes, mut fuel)) = query_player.single_mut() {
        let direction = keyboard_input
            .get_pressed()
            .fold(Vec2::ZERO, |mut direction, key| {
                match key {
                    KeyCode::ArrowLeft => direction.x -= 1.0,
                    KeyCode::ArrowRight => direction.x += 1.0,
                    KeyCode::ArrowUp => direction.y += 1.0,
                    _ => (),
                }
                direction
            });
        if direction != Vec2::ZERO {
            if direction.x != 0.0 {
                velocity.linvel.x = direction.x * attributes.ground_speed_factor;
            }
            if direction.y != 0.0 {
                velocity.linvel.y = direction.y * attributes.flying_speed_factor;
                *drill_state = DrillState::Flying;
            }
            fuel.current -= (1.0 / attributes.fuel_efficiency) * time.delta_secs();
        }
    }
}

pub fn update_player_on_state_changes(
    mut query: Query<
        (&DrillState, &mut Damping, &mut Sprite),
        (With<Player>, Changed<DrillState>),
    >,
) {
    if let Ok((state, mut damping, mut sprite)) = query.single_mut() {
        debug!(
            "update_player_on_state_changes {{ DrillState: {:?}, Damping: {:?} }}",
            state, damping
        );
        if *state == DrillState::Idle {
            damping.linear_damping = 10.0;
        } else {
            damping.linear_damping = 0.5;
        }
        if let Some(texture_sprite) = &mut sprite.texture_atlas {
            match state {
                DrillState::Idle | DrillState::Falling => texture_sprite.index = 0,
                DrillState::Flying => texture_sprite.index = 1,
                DrillState::Drilling => texture_sprite.index = 2,
            };
        };
    }
}

pub fn update_player_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut PlayerDirection, &mut Sprite), With<Player>>,
) {
    if let Ok((_transform, mut direction, mut sprite)) = player_query.single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if *direction != PlayerDirection::Left {
                *direction = PlayerDirection::Left;
                sprite.flip_x = true;
            }
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            if *direction != PlayerDirection::Right {
                *direction = PlayerDirection::Right;
                sprite.flip_x = false;
            }
        }
    }
}

pub fn falling_detection(
    mut player_query: Query<(&Velocity, &Transform, &mut DrillState), With<Player>>,
    read_rapier_context: ReadRapierContext,
) {
    if let Ok((velocity, transform, mut drill_state)) = player_query.single_mut() {
        let player_pos = transform.translation.truncate();

        if let Ok(context) = read_rapier_context.single() {
            if let Some((_, toi)) = context.cast_shape(
                player_pos,
                0.0,
                Vec2::NEG_Y,
                &*SharedShape::cuboid(8.0, 16.0),
                ShapeCastOptions {
                    stop_at_penetration: false,
                    ..default()
                },
                QueryFilter::default(),
            ) {
                if toi.time_of_impact > 10.0 && velocity.linvel.y < -1.0 {
                    *drill_state = DrillState::Falling;
                }
            }
        }
    }
}
