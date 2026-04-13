use crate::BlackQuartzCamera;
use crate::game::GameState::Playing;
use crate::game::GameSystems::Rendering;
use crate::map::WorldGrid;
use crate::prelude::{DrillState, LoadingProgress, Player};
use bevy::math::Vec2;
use bevy::prelude::{
    App, Camera2d, Commands, IntoScheduleConfigs, OrthographicProjection, Plugin, Projection,
    Query, Res, ResMut, Startup, Time, Transform, Update, With, Without, in_state,
};
use bevy::prelude::FloatExt;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            follow_player.in_set(Rendering).run_if(in_state(Playing)),
        );
    }
}

fn setup_camera(mut commands: Commands, mut loading_progress: ResMut<LoadingProgress>) {
    commands.spawn((
        Camera2d::default(),
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: Default::default(),
            scale: 0.45,
            area: Default::default(),
        }),
        BlackQuartzCamera,
    ));
    loading_progress.init_camera = true;
}

fn follow_player(
    time: Res<Time>,
    query_player: Query<(&Transform, &DrillState), With<Player>>,
    mut query_camera: Query<
        (&mut Transform, &Projection),
        (With<BlackQuartzCamera>, Without<Player>),
    >,
    world_grid: Res<WorldGrid>,
) {
    // Camera handling
    if let Ok((player_transform, _drill_state)) = query_player.single() {
        let player_pos = player_transform.translation;
        if let Ok((mut camera_pos, camera)) = query_camera.single_mut() {
            if let Projection::Orthographic(ortho) = camera {
                let camera_area = ortho.area;
                let t = (5.0_f32 * time.delta_secs()).min(1.0_f32);

                if player_pos.x + camera_area.max.x <= world_grid.map_area.max.x
                    && player_pos.x + camera_area.min.x >= world_grid.map_area.min.x
                {
                    camera_pos.translation.x = camera_pos.translation.x.lerp(player_pos.x, t);
                }

                if player_pos.y + camera_area.max.y <= world_grid.map_area.max.y
                    && player_pos.y + camera_area.min.y >= world_grid.map_area.min.y
                {
                    camera_pos.translation.y = camera_pos.translation.y.lerp(player_pos.y, t);
                }
            }
        }
    }
}
