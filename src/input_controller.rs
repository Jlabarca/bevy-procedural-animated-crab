/**
 * Todo: Fix gravity issue, hardcoded to 0 in wanderluster
 * Prob creando controller settings propio
 */
use bevy::prelude::*;
// use bevy_editor_pls::controls::{Action, Binding, Button, EditorControls, UserInput};
// use bevy_editor_pls::prelude::*;
use bevy_mod_wanderlust::{ControllerInput, WanderlustPlugin};

pub struct InputControllerPlugin;

impl Plugin for InputControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WanderlustPlugin)
            .add_system_to_stage(CoreStage::PreUpdate, input_system);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Player;
const SPEED: f32 = 0.05;
fn input_system(
    mut body: Query<(&mut ControllerInput, &GlobalTransform)>,
    input: Res<Input<KeyCode>>,
    // mut mouse: EventReader<MouseMotion>,
    // time: Res<Time>,
) {
    // const SENSITIVITY: f32 = 0.025;
    // const ROLL_MULT: f32 = 5.0;

    let (mut body, tf) = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::J) {
        dir += -tf.right() * SPEED;
    }
    if input.pressed(KeyCode::L) {
        dir += tf.right() * SPEED;
    }
    if input.pressed(KeyCode::K) {
        dir += -tf.forward() * SPEED;
    }
    if input.pressed(KeyCode::I) {
        dir += tf.forward() * SPEED;
    }
    if input.pressed(KeyCode::O) {
        dir += -tf.up() * SPEED;
    }
    if input.pressed(KeyCode::P) {
        dir += tf.up() * SPEED;
    }

    body.movement = dir;

    // let dt = time.delta_seconds();
    // for &MouseMotion { delta } in mouse.iter() {
    //     body.torque += tf.up() * -delta.x * dt * SENSITIVITY;
    //     body.torque_impulse += tf.right() * -delta.y * dt * SENSITIVITY;
    // }
    // if input.pressed(KeyCode::Q) {
    //     body.torque_impulse += -tf.forward() * dt * SENSITIVITY * ROLL_MULT;
    // }
    // if input.pressed(KeyCode::E) {
    //     body.torque_impulse += tf.forward() * dt * SENSITIVITY * ROLL_MULT;
    // }
}
