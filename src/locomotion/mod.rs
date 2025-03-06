use bevy::prelude::*;
use bevy::animation::AnimationPlayer;

pub mod data;
pub mod analyzer;
pub mod blender;
pub mod controller;

pub use data::*;

/// Plugin for the locomotion system
pub struct LocomotionPlugin;

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_character_movement,
            update_animation_state,
        ));
    }
}

/// Updates character position based on controller input
fn update_character_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CharacterController)>,
) {
    for (mut transform, controller) in query.iter_mut() {
        // Simple movement for now - we'll enhance this later
        let velocity = controller.velocity * time.delta_secs();
        transform.translation += velocity;
        
        // Handle rotation
        if controller.rotational_velocity != 0.0 {
            let rotation = Quat::from_rotation_y(controller.rotational_velocity * time.delta_secs());
            transform.rotation = rotation * transform.rotation;
        }
    }
}

/// Updates character animation state
fn update_animation_state(
    _query: Query<(&CharacterController, &Children)>,
    _animation_player_query: Query<&mut AnimationPlayer>,
    _children_query: Query<&Children>,
) {
    // We'll implement animation blending and transitions here later
    // For now, just a placeholder
}
