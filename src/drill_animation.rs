use crate::game::GameSystems::Animation;
use crate::prelude::*;
use bevy::app::App;
use bevy::math::VectorSpace;
use bevy::prelude::{
    Component, IntoSystemConfigs, Plugin, Query, Res, Time, Transform, Update, Vec3, With, info,
};
use bevy::reflect::impl_from_reflect_opaque;

pub struct DrillAnimationPlugin;

impl Plugin for DrillAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_drilling.in_set(Animation));
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
            drilling_bob_amplitude: 1.0,  // pixel di movimento
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
    shake_pattern: ShakePattern,
}
impl Default for DrillShake {
    fn default() -> Self {
        Self {
            intensity: 1.0,
            frequency: 50.0,
            shake_pattern: ShakePattern::Circular,
        }
    }
}

#[derive(Clone, Copy)]
enum ShakePattern {
    Uniform,    // Shake uniforme in tutte le direzioni
    Horizontal, // Solo orizzontale
    Vertical,   // Solo verticale
    Circular,   // Movimento circolare
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
                if let Some(base_pos) = drill_animation.base_position {
                    drill_animation.base_position = None;
                    drill_animation.internal_timer = 0.0;
                }
            }
            _ => {}
        }
    }
}
