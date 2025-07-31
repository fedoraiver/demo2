use bevy::prelude::*;
use bevy_trauma_shake::*;

pub fn camera_shake(mut shake: Single<&mut Shake>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        shake.add_trauma(0.2);
    }
}
