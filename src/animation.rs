use crate::BlackQuartzCamera;
use crate::game::GameSystems::Animation;
use crate::prelude::*;
use bevy::app::App;
use bevy::prelude::{
    Commands, Component, Entity, IntoScheduleConfigs, Plugin, Query, Res, Time, Timer, Transform,
    Update, Vec3, With, debug,
};
use rand::Rng;

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_drilling.in_set(Animation))
            .add_systems(Update, animate_camera.in_set(Animation));
    }
}

#[derive(Component)]
#[require(DrillShake)]
pub struct DrillAnimation {
    drilling_bob_amplitude: f32,
    drilling_bob_frequency: f32,
    // Memorizzo la posizione di base per evitare accumulo di offset
    base_position: Option<Vec3>,
    // Timer interno per animazioni più complesse
    internal_timer: f32,
}
impl Default for DrillAnimation {
    fn default() -> Self {
        Self {
            drilling_bob_amplitude: 0.5,  // pixel di movimento
            drilling_bob_frequency: 60.0, // oscillazioni per secondo
            base_position: None,
            internal_timer: 0.0,
        }
    }
}
#[derive(Component)]
struct DrillShake {
    intensity: f32,
    frequency: f32,
}
impl Default for DrillShake {
    fn default() -> Self {
        Self {
            intensity: 0.5,
            frequency: 60.0,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct CameraShake {
    pub base_position: Option<Vec3>,
    pub intensity: f32,
    pub timer: Timer,
}

fn animate_drilling(
    time: Res<Time>,
    mut drill_query: Query<
        (
            &mut Transform,
            &DrillState,
            &mut DrillAnimation,
            Option<&DrillShake>,
        ),
        With<Player>,
    >,
) {
    for (mut transform, drill_state, mut drill_animation, shake) in &mut drill_query {
        let dt = time.delta_secs();
        drill_animation.internal_timer += dt;

        match drill_state {
            DrillState::Drilling => {
                //info!("Drilling animation");
                // Salva la posizione base se non l'abbiamo ancora fatto
                if drill_animation.base_position.is_none() {
                    drill_animation.base_position = Some(transform.translation);
                }
                let base_pos = drill_animation.base_position.unwrap();

                // Movimento su e giù (bob) durante il drilling
                let bob_offset =
                    (drill_animation.internal_timer * drill_animation.drilling_bob_frequency).sin()
                        * drill_animation.drilling_bob_amplitude;

                // Shake laterale per dare impatto
                let final_shake = if let Some(shake) = shake {
                    let shake_x =
                        (drill_animation.internal_timer * shake.frequency).sin() * shake.intensity;
                    let shake_y = (drill_animation.internal_timer * shake.frequency * 1.3).cos()
                        * shake.intensity
                        * 0.7;

                    // Leggero effetto di "impatto" periodico
                    let impact_intensity = if (drill_animation.internal_timer * 3.0).sin() > 0.8 {
                        2.0 * shake.intensity
                    } else {
                        shake.intensity
                    };
                    let shake_offset = Vec3::new(shake_x, shake_y, 0.0);
                    shake_offset * impact_intensity
                } else {
                    Vec3::ZERO
                };

                transform.translation =
                    base_pos + Vec3::new(final_shake.x, bob_offset + final_shake.y, 0.0);
            }
            DrillState::Idle => {
                // Ritorna alla posizione base e resetta il timer
                if drill_animation.base_position.is_some() {
                    drill_animation.base_position = None;
                    drill_animation.internal_timer = 0.0;
                }
            }
            _ => {}
        }
    }
}

fn animate_camera(
    mut commands: Commands,
    time: Res<Time>,
    mut camera_query: Query<
        (Entity, &mut Transform, Option<&mut CameraShake>),
        With<BlackQuartzCamera>,
    >,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut transform, mut shake_option) in &mut camera_query {
        if let Some(shake) = shake_option.as_mut() {
            // Salva la posizione base se non l'abbiamo ancora fatto
            if shake.base_position.is_none() {
                shake.base_position = Some(transform.translation);
            }
            debug!("Camera shake {:?}", shake);
            shake.timer.tick(time.delta());
            if shake.timer.finished() {
                // Shake finito: resetta posizione
                transform.translation.x = shake.base_position.unwrap().x;
                transform.translation.y = shake.base_position.unwrap().y;
                commands.entity(entity).remove::<CameraShake>();
            } else {
                // Offset casuale nell'intervallo [-intensity, intensity]
                let offset_x = rng.gen_range(-shake.intensity..shake.intensity);
                let offset_y = rng.gen_range(-shake.intensity..shake.intensity);
                transform.translation.x = transform.translation.x + offset_x;
                transform.translation.y = transform.translation.y + offset_y;
            }
        }
    }
}
