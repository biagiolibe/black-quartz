mod game;
mod loading;
mod menu;
mod player;

mod prelude {
    pub use crate::game::*;
    pub use crate::loading::*;
    pub use crate::menu::*;
    pub use crate::player::*;
    pub const TILE_SIZE: f32 = 32.0;
}

use crate::game::GamePlugin;
use crate::prelude::TILE_SIZE;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::camera::SubCameraView;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct BlackQuartzCamera;

#[derive(Component)]
struct Tile;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Black Quartz".to_string(),
                    mode: WindowMode::Windowed,
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(GamePlugin)
        .add_systems(Startup, (setup, setup_borders))
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let tile_size = Vec2::new(TILE_SIZE, TILE_SIZE);

    // Crea un'immagine vuota per il texture atlas
    let image = Image::new_fill(
        Extent3d {
            width: tile_size.x as u32,
            height: tile_size.y as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[139, 69, 19, 255], // RGBA (brown)
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let handle_image = images.add(image);

    let window = windows.single();
    // Ground
    let tiles_number_x = (window.resolution.width() / TILE_SIZE) as i32;
    let tiles_number_y = (window.resolution.height() / TILE_SIZE) as i32;
    for x in -tiles_number_x..tiles_number_x {
        for y in 0..tiles_number_y {
            commands.spawn((
                Sprite {
                    image: handle_image.clone(),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * tile_size.x,
                    (y as f32 * tile_size.y) * (-1.0),
                    0.0,
                ),
                GlobalTransform::default(),
                RigidBody::Fixed,
                Collider::cuboid(TILE_SIZE / 2f32, TILE_SIZE / 2f32),
                Tile,
            ));
        }
    }
    println!("window x: {}, y: {}", window.resolution.width(), window.resolution.height());

    println!("window offset x: {}, y: {}", window.resolution.width() / 8., window.resolution.height() / 8.);
    //TODO move to a plugin?
    // Camera
    commands.spawn((
                       Camera2d,
                       Camera {
                           order: 0,
                           sub_camera_view: Some(SubCameraView {
                               full_size: UVec2::new(window.resolution.width() as u32, window.resolution.height() as u32),
                               offset: Vec2::new(window.resolution.width() / 8., window.resolution.height() / 8.),
                               size: UVec2::new(window.resolution.width() as u32 / 2, window.resolution.height() as u32 / 2),
                               ..default()
                           }),
                           ..default()
                       },
                       BlackQuartzCamera,
                   ));
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
