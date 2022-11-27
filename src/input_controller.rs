
use bevy::prelude::*;
// use bevy_editor_pls::controls::{Action, Binding, Button, EditorControls, UserInput};
// use bevy_editor_pls::prelude::*;
use bevy_mod_wanderlust::{
    ControllerInput, WanderlustPlugin
};



pub struct InputControllerPlugin;

impl Plugin for InputControllerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(WanderlustPlugin)
        .add_system_to_stage(CoreStage::PreUpdate, input)
        ;
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Player;

fn input(
    mut body: Query<(&mut ControllerInput, &GlobalTransform)>,
    input: Res<Input<KeyCode>>,
    // mut mouse: EventReader<MouseMotion>,
    // time: Res<Time>,
) {
    // const SENSITIVITY: f32 = 0.025;
    // const ROLL_MULT: f32 = 5.0;

    let (mut body, tf) = body.single_mut();

    let mut dir = Vec3::ZERO;
    if input.pressed(KeyCode::A) {
        dir += -tf.right();
    }
    if input.pressed(KeyCode::D) {
        dir += tf.right();
    }
    if input.pressed(KeyCode::S) {
        dir += -tf.forward();
    }
    if input.pressed(KeyCode::W) {
        dir += tf.forward();
    }
    if input.pressed(KeyCode::LControl) {
        dir += -tf.up();
    }
    if input.pressed(KeyCode::Space) {
        dir += tf.up();
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