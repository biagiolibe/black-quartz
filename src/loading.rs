use crate::prelude::*;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), load_assets);
    }
}

#[derive(Resource)]
struct ImageResource(Handle<Image>);

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    /*
    let handle_image: Handle<Image> = asset_server.load("textures/asset.png");
    {
        commands.insert_resource(ImageResource(handle_image));
    }

     */
}