use bevy::prelude::*;

use super::core::*;

pub fn player_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }

    for mut transform in query.iter_mut() {
        transform.translation += direction * 4.;
    }
}
