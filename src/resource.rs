use crate::game::GameState::Playing;
use crate::prelude::GameState::{Loading, MainMenu, Rendering};
use crate::prelude::*;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct LoadingProgress {
    pub loading_assets: bool,
    pub rendering_map: bool,
    pub spawning_player: bool,
    pub spawning_base: bool,
    pub init_camera: bool,
}
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingProgress>()
            .add_systems(OnEnter(Loading), load_assets.in_set(GameSystems::Loading))
            .add_systems(Update, check_loading_progress.run_if(in_state(Rendering)));
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub buildings: AssetTexture,
    pub player: AssetTexture,
    pub terrain: AssetTexture,
}

pub struct AssetTexture {
    pub texture: Handle<Image>,
    pub texture_layout: Handle<TextureAtlasLayout>,
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut loading_progress: ResMut<LoadingProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Loading assets");
    //Terrain assets
    let terrain_texture_handle: Handle<Image> = asset_server.load("textures/terrain.png");
    let terrain_layout = TextureAtlasLayout::from_grid(UVec2::new(150, 150), 4, 4, None, None);
    let terrain_layout_handle = texture_atlas_layouts.add(terrain_layout);

    //Player assets
    let player_texture_handle: Handle<Image> =
        asset_server.load("textures/drilling.png");
    let player_texture_layout =
        TextureAtlasLayout::from_grid(UVec2::new(750, 830), 2, 3, None, None);
    let player_layout_handle = texture_atlas_layouts.add(player_texture_layout);

    //Buildings assets
    let buildings_texture_handle: Handle<Image> = asset_server.load("textures/buildings.png");
    let buildings_texture_layout =
        TextureAtlasLayout::from_grid(UVec2::new(911, 727), 1, 1, None, None);
    let buildings_layout_handle = texture_atlas_layouts.add(buildings_texture_layout);

    commands.insert_resource(GameAssets {
        player: AssetTexture {
            texture: player_texture_handle,
            texture_layout: player_layout_handle,
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
    info!("Loading complete");
    loading_progress.loading_assets = true;
    next_state.set(MainMenu);
}

pub fn check_loading_progress(
    loading_progress: Res<LoadingProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if loading_progress.loading_assets
        && loading_progress.rendering_map
        && loading_progress.spawning_player
        && loading_progress.spawning_base
        && loading_progress.init_camera
    {
        next_state.set(Playing);
    }
}
