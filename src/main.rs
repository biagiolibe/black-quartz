mod game;
mod loading;
mod player;
mod menu;

mod prelude {
    pub use crate::game::*;
    pub use crate::loading::*;
    pub use crate::menu::*;
    pub use crate::player::*;
}

use crate::game::GamePlugin;
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::WindowMode;
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
                    title: "Drilly McDrillface".to_string(),
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
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {

    let tile_size = Vec2::new(64.0, 64.0);

    // Crea un'immagine vuota (bianca) per il texture atlas
    let mut image = Image::new_fill(
        Extent3d {
            width: tile_size.x as u32,
            height: tile_size.y as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[139, 69, 19, 255], // RGBA (marrone)
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default()
    );

    let handle_image= images.add(image);

    // Ground
    for x in 0..4 {
        for y in 0..4 {
            commands.spawn((
                Sprite {
                    image: handle_image.clone(),
                    ..default()
                },
                Transform::from_xyz(x as f32 * tile_size.x, (y as f32 * tile_size.y )*(-1.0), 0.0),
                GlobalTransform::default(),
                RigidBody::Fixed,
                Collider::cuboid(32.0, 32.0),
                Tile
            ));
        }
    }

    // Camera
    commands.spawn(
        (Camera2d::default(),
         BlackQuartzCamera,
        )
    );
}

