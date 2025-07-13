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
    pub buildings: AssetTexture,
    pub player: AssetTexture,
    pub terrain: AssetTexture,
}

pub struct AssetTexture{
    pub texture: Handle<Image>,
    pub texture_layout: Handle<TextureAtlasLayout>,
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Loading assets");
    //Terrain assets
    let terrain_texture_handle: Handle<Image> = asset_server.load("textures/terrain.png");
    let terrain_layout = TextureAtlasLayout::from_grid(
        UVec2::new(150, 150),
        4,
        4,
        None,
        None,
    );
    let terrain_layout_handle = texture_atlas_layouts.add(terrain_layout);

    //Player assets
    let player_texture_handle: Handle<Image> = asset_server.load("textures/drilling_machine_full.png");
    let player_texture_layout = TextureAtlasLayout::from_grid(
        UVec2::new(350, 383),
        2,
        4,
        None,
        None,
    );
    let player_layout_handle = texture_atlas_layouts.add(player_texture_layout);

    //Buildings assets
    let buildings_texture_handle: Handle<Image> = asset_server.load("textures/buildings.png");
    let buildings_texture_layout = TextureAtlasLayout::from_grid(
        UVec2::new(1024, 1024),
        1,
        1,
        None,
        None,
    );
    let buildings_layout_handle = texture_atlas_layouts.add(buildings_texture_layout);

    commands.insert_resource(GameAssets{
        player: AssetTexture {
            texture: player_texture_handle,
            texture_layout:player_layout_handle,
        },
        terrain: AssetTexture {
            texture: terrain_texture_handle,
            texture_layout: terrain_layout_handle,
        },
        buildings: AssetTexture {
            texture: buildings_texture_handle,
            texture_layout: buildings_layout_handle,
        },
    });
    println!("Loading complete");
    next_state.set(GameState::Playing);
}
