use crate::BlackQuartzCamera;
use crate::game::GameState::Playing;
use crate::map::WorldGrid;
use crate::prelude::{LoadingProgress, Player, move_player, DrillState};
use bevy::math::Vec2;
use bevy::prelude::{
    App, Camera2d, Commands, IntoSystemConfigs, OrthographicProjection, Plugin, Query, Res, ResMut,
    Startup, Transform, Update, With, Without, in_state,
};
use crate::game::GameSystems::Rendering;
use crate::prelude::GameSystems::Running;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, follow_player.in_set(Rendering).run_if(in_state(Playing)));
    }
}

fn setup_camera(mut commands: Commands, mut loading_progress: ResMut<LoadingProgress>) {
    commands.spawn((
        Camera2d::default(),
        OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: Default::default(),
            scale: 0.45,
            area: Default::default(),
        },
        BlackQuartzCamera,
    ));
    loading_progress.init_camera = true;
}

fn follow_player(
    query_player: Query<(&Transform, &DrillState), With<Player>>,
    mut query_camera: Query<
        (&mut Transform, &OrthographicProjection),
        (With<BlackQuartzCamera>, Without<Player>),
    >,
    world_grid: Res<WorldGrid>,
) {
    // Camera handling
    if let Ok((player_transform, drill_state)) = query_player.get_single() {
        if matches!(drill_state, DrillState::Drilling) {
           return; 
        }
        let player_pos = player_transform.translation;
        let (mut camera_pos, camera) = query_camera.single_mut();
        let camera_area = &camera.area;

        if player_pos.x + camera_area.max.x <= world_grid.map_area.max.x
            && player_pos.x + camera_area.min.x >= world_grid.map_area.min.x
        {
            camera_pos.translation.x = player_pos.x;
        }

        if player_pos.y + camera_area.max.y <= world_grid.map_area.max.y
            && player_pos.y + camera_area.min.y >= world_grid.map_area.min.y
        {
            camera_pos.translation.y = player_pos.y;
        }
    }
}
