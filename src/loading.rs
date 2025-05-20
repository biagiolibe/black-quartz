use crate::map::TILE_SIZE;
use crate::prelude::*;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub texture: Handle<Image>,
    pub texture_layout: Handle<TextureAtlasLayout>,
    pub terrain_texture: Handle<Image>,
    pub terrain_texture_layout: Handle<TextureAtlasLayout>,
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Loading assets");
    //Terrain assets
    let terrain_texture_handle: Handle<Image> = asset_server.load("textures/tileset.png");
    let terrain_layout = TextureAtlasLayout::from_grid(
        UVec2::new(500, 500),
        1,
        1,
        None,
        None,
    );
    let terrain_layout_handle = texture_atlas_layouts.add(terrain_layout);

    //Other assets
    let texture_handle: Handle<Image> = asset_server.load("textures/drilling_machine.png");
    let texture_layout = TextureAtlasLayout::from_grid(
        UVec2::new(512, 512),
        2,
        1,
        None,
        None,
    );
    let texture_layout_handle = texture_atlas_layouts.add(texture_layout);

    commands.insert_resource(GameAssets{
        texture:texture_handle,
        texture_layout:texture_layout_handle,
        terrain_texture: terrain_texture_handle,
        terrain_texture_layout: terrain_layout_handle
    });
    println!("Loading complete");
    next_state.set(GameState::Playing);
}
