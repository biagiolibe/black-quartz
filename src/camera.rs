use crate::BlackQuartzCamera;
use crate::prelude::{Player, move_player};
use bevy::math::Vec2;
use bevy::prelude::{
    App, Camera, Camera2d, Commands, IntoSystemConfigs, OrthographicProjection, Plugin, Query,
    Startup, Transform, Update, Vec3, Window, With, Without,
};
use bevy::window::PrimaryWindow;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, follow_player.after(move_player));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: Default::default(),
            scale: 0.6,
            area: Default::default(),
        },
        BlackQuartzCamera,
    ));
}

fn follow_player(
    query_player: Query<&Transform, With<Player>>,
    mut query_camera: Query<(&mut Transform, &Camera), (With<BlackQuartzCamera>, Without<Player>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    // Camera handling
    let window = windows.single();
    if let Ok(player_transform) = query_player.get_single() {
        let player_pos = player_transform.translation;
        let (mut camera_pos, camera) = query_camera.single_mut();
        let x_pos = (player_pos.x).min(window.width());
        let y_pos = (player_pos.y).min(window.height());
        camera_pos.translation = Vec3::new(x_pos, y_pos, camera_pos.translation.z);
    }
}

fn setup_borders(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    //Top border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(window.resolution.width() / 2.0, 5.0),
        Transform::from_xyz(0.0, window.height() / 2.0, 0.0),
    ));

    //Bottom border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(window.resolution.width() / 2.0, 5.0),
        Transform::from_xyz(0.0, -(window.height() / 2.0), 0.0),
    ));

    //Left border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(5.0, window.resolution.height() / 2.0),
        Transform::from_xyz(-(window.width() / 2.0), 0.0, 0.0),
    ));

    //Right border
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(5.0, window.resolution.height() / 2.0),
        Transform::from_xyz(window.width() / 2.0, 0.0, 0.0),
    ));
}
