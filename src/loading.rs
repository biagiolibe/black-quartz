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
    pub(crate) texture: Handle<Image>,
    pub(crate) layout: Handle<TextureAtlasLayout>,
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("Loading assets");
    let texture_handle: Handle<Image> = asset_server.load("textures/tileset.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(512, 512),
        2,
        2,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.insert_resource(GameAssets{
        texture:texture_handle,
        layout:layout_handle,
    });
    println!("Loading complete");
    next_state.set(GameState::Playing);
}
