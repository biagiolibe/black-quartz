use crate::prelude::*;
use bevy::app::App;
use bevy::prelude::{Component, Plugin, Query, Res, Time, Transform, Update, Vec3, With, info};

pub struct DrillAnimationPlugin;

impl Plugin for DrillAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_drilling);
    }
}

#[derive(Component)]
#[require(DrillShake)]
pub struct DrillAnimation {
    drilling_bob_amplitude: f32,
    drilling_bob_frequency: f32,
}
impl Default for DrillAnimation {
    fn default() -> Self {
        Self {
            drilling_bob_amplitude: 1.0,  // pixel di movimento
            drilling_bob_frequency: 70.0, // oscillazioni per secondo
        }
    }
}
#[derive(Component)]
struct DrillShake {
    intensity: f32,
    frequency: f32,
    timer: f32,
}
impl Default for DrillShake {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            frequency: 70.0,
            timer: 0.0,
        }
    }
}

fn animate_drilling(
    time: Res<Time>,
    mut drill_query: Query<
        (
            &mut Transform,
            &DrillState,
            &DrillAnimation,
            Option<&DrillShake>,
        ),
        With<Player>,
    >,
) {
    for (mut transform, drill_state, drill, shake) in &mut drill_query {
        match drill_state {
            DrillState::Drilling => {
                info!("Drilling animation");
                let dt = time.delta_secs();
                // Movimento su e giù (bob) durante il drilling
                let bob_offset = (time.elapsed_secs() * drill.drilling_bob_frequency).sin()
                    * drill.drilling_bob_amplitude;

                // Shake laterale per dare impatto
                let shake_offset = if let Some(shake) = shake {
                    let shake_x = (time.elapsed_secs() * shake.frequency).sin() * shake.intensity;
                    let shake_y =
                        (time.elapsed_secs() * shake.frequency * 1.3).cos() * shake.intensity * 0.5;
                    Vec3::new(shake_x, shake_y, 0.0)
                } else {
                    Vec3::ZERO
                };

                let original_pos = transform.translation.clone();
                transform.translation =
                    original_pos + Vec3::new(0.0, bob_offset, 0.0) + shake_offset;
            }
            _ => {}
        }
    }
}
