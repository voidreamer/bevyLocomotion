use bevy::prelude::*;
use crate::locomotion::data::*;

/// System for handling player input to control the character

pub fn player_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CharacterController>,
    _time: Res<Time>,
) {
    for mut controller in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        // Check movement keys
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        
        // Normalize direction if necessary
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }
        
        // Set velocity (with a base speed)
        let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) { 5.0 } else { 2.5 };
        controller.velocity = direction * speed;
        
        // Handle rotation
        controller.rotational_velocity = 0.0;
        if keyboard_input.pressed(KeyCode::KeyQ) {
            controller.rotational_velocity = 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            controller.rotational_velocity = -1.0;
        }
    }
}
