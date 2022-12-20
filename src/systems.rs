use std::{ops::Mul, time::Duration};

use crate::components::{Foot, FootAnchor, FootPole, FootTarget, Ground, MoveAnchorEvent, Player};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::{
    KinematicCharacterController, KinematicCharacterControllerOutput, QueryFilter, RapierContext,
    Real,
};
use bevy_tweening::{lens::*, *};


pub fn anchor_move_event_trigger_system(
    time: Res<Time>,
    foot_targets: Query<(Entity, &FootTarget, &GlobalTransform), Without<FootAnchor>>,
    mut anchor_query: Query<
        (Entity, &mut FootAnchor, &GlobalTransform),
        (With<FootAnchor>, Without<FootTarget>),
    >,
    player_query: Query<&Player>,
    mut move_event_writer: EventWriter<MoveAnchorEvent>,
) {
    for (target_entity, foot_target, target_transform) in foot_targets.iter() {
        if let Ok((_, mut anchor, anchor_transform)) = anchor_query.get_mut(foot_target.anchor) {
            let player = player_query.get(foot_target.owner).unwrap();
            let distance = anchor_transform
                .translation()
                .xz()
                .distance(target_transform.translation().xz()); //only xz distance

            anchor
                .animation_timer
                .tick(Duration::from_secs_f32(time.delta().as_secs_f32()));

            if anchor.animation_timer.finished() {
                anchor.moving = false;
            }

            if anchor.moving == true {
                continue;
            }

            if distance > player.walk_width {
                // not using anchor.max_distance anymore for debugging purposes
                let speed = 2.0
                    + 15.0
                        * (player.current_speed.x.powi(2) + player.current_speed.z.powi(2))
                            .sqrt()
                            .powi(2);
                move_event_writer.send(MoveAnchorEvent {
                    anchor: foot_target.anchor,
                    target: target_entity,
                    animation_duration: Duration::from_secs_f32(
                        anchor.animation_duration.as_secs_f32() / speed,
                    ),
                });

                anchor.moving = true;
                // info!("Triggering foot {:?}", anchor.max_distance);
                // info!("foot.desired_pos: {:?}", target_transform.translation());
            }
        }
    }
}

pub fn anchor_move_event_system(
    mut commands: Commands,
    mut reader: EventReader<MoveAnchorEvent>,
    mut anchor_query: Query<(&mut FootAnchor, &Transform), Without<FootTarget>>,
    target_query: Query<(&FootTarget, &GlobalTransform)>,
    player_query: Query<&Player>,
) {
    for event in reader.iter() {
        if let Ok((mut anchor, anchor_transform)) = anchor_query.get_mut(event.anchor) {
            if let Ok((target, target_global_transform)) = target_query.get(event.target) {
                let mut target_position = target_global_transform.translation().clone();

                if anchor.inverted {
                    if let Ok(player) = player_query.get(target.owner) {
                        target_position += Vec3::new(0.0, player.walk_height, 0.0);
                    }
                }

                anchor.inverted = !anchor.inverted;

                let tween = Tween::new(
                    EaseFunction::BounceInOut,
                    event.animation_duration,
                    TransformPositionLens {
                        start: anchor_transform.translation,
                        end: target_position,
                    },
                );

                commands.entity(event.anchor).insert(Animator::new(tween));
                anchor.animation_timer.reset();
            }
        }
    }
}

// foot target at body side
pub fn target_at_side_system(
    mut foot_targets: Query<(&FootTarget, &mut Transform), Without<Player>>,
    player_query: Query<(&Player, &Transform)>,
) {
    for (foot_target, mut target_transform) in foot_targets.iter_mut() {
        if let Ok((player, player_transform)) = player_query.get(foot_target.owner) {
            target_transform.translation.x =
                player_transform.translation.x + foot_target.pos_offset.x * player.walk_spread.x;
            target_transform.translation.z =
                player_transform.translation.z + foot_target.pos_offset.z * player.walk_spread.z;
        }
    }
}

pub fn target_height_system(
    mut foot_targets: Query<(Entity, &FootTarget, &mut Transform), Without<Player>>,
    ground_query: Query<&Ground>,
    name_query: Query<&Name>,
    rapier_context: Res<RapierContext>,
    mut move_event_writer: EventWriter<MoveAnchorEvent>,
) {
    for (foot_target_entity, foot_target, mut target_transform) in foot_targets.iter_mut() {
        //Calculate height using raycast
        let ray_pos = target_transform.translation + Vec3::new(0.0, 1.0, 0.0);
        let ray_dir = Vec3::new(0.0, -1.0, 0.0);
        let max_toi = Real::MAX;
        let solid = true;
        let filter = QueryFilter::default().exclude_collider(foot_target.owner);
        if let Some((e, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
            let hit_point = ray_pos + ray_dir * toi;
            target_transform.translation.y = hit_point.y;
            //target_transform.translation.y = hit_point.y;
            if let Err(ground) = ground_query.get(e) {
                if let Ok(name) = name_query.get(e) {
                    target_transform.translation.y += 0.2; //todo: check why hit_point.y is not enough
                    //info!("hit_point: {:?} at {:?}", name, hit_point);
                    move_event_writer.send(
                        MoveAnchorEvent {
                            anchor: foot_target.anchor,
                            target: foot_target_entity,
                            animation_duration: Duration::from_secs_f32(0.02),
                        }
                    );
                }
            }
        }
    }
}

// }
// for (foot_target_entity, foot_target, mut target_transform) in foot_targets.iter_mut() {
//     //Calculate height using raycast

//     let shape = Collider::cuboid(0.33, 1.0, 0.33);
//     let shape_pos = target_transform.translation + Vec3::new(0.0, -0.5, 0.0);
//     let shape_rot = Quat::from_rotation_z(0.0);
//     let shape_vel = Vec3::new(0.0, -0.4, 0.0);
//     let max_toi = 4.0;
//     let filter = QueryFilter::default().exclude_collider(foot_target.owner); //filtering crab body

//     if let Some((entity, hit)) = rapier_context.cast_shape(
//         shape_pos, shape_rot, shape_vel, &shape, max_toi, filter
//     ) {
//         println!("Hit the entity {:?} with the configuration: {:?}", entity, hit);
//         info!("hit_point: {:?}", hit.);
//     }
// }

pub fn pole_system(
    mut foot_poles: Query<(&FootPole, &mut Transform), Without<Player>>,
    player_query: Query<(&Player, &Transform), Without<FootPole>>,
) {
    for (foot_pole, mut target_transform) in foot_poles.iter_mut() {
        if let Ok((player, player_transform)) = player_query.get(foot_pole.owner) {
            target_transform.translation = player_transform.translation
                + (foot_pole.pos_offset + player.pole_offset) * player.pole_spread;
        }
    }
}

// keeping foot on anchor for debugging purposes //todo: foot transform is local, anchor is global so this is not working rn
pub fn force_foot_on_anchor_system(
    mut anchor_query: Query<(&FootAnchor, &mut Transform), Without<Foot>>,
    mut foot_query: Query<&mut Transform, (With<Foot>, Without<FootAnchor>)>,
    mut target_query: Query<&FootTarget>,
) {
    for (anchor, mut anchor_transform) in anchor_query.iter_mut() {
        if let Ok(mut foot_transform) = foot_query.get_mut(anchor.foot.unwrap()) {
            if !anchor.moving {
                foot_transform.translation = foot_transform.translation;
            }
        }
    }
}

pub fn handle_move(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut Player,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
) {
    let (mut transform, mut player, mut controller, controller_output) = query.single_mut();
    let mut desired_movement = Vec3::ZERO;
    let mut speed = player.move_speed;
    let x = Vec3::new(1.0, 0.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);

    for key in input.get_pressed() {
        match *key {
            KeyCode::D => {
                desired_movement += x;
            }
            KeyCode::A => {
                desired_movement -= x;
            }
            KeyCode::W => {
                desired_movement -= z;
            }
            KeyCode::S => {
                desired_movement += z;
            }
            KeyCode::Space => {
                if player.grounded {
                    player.jumping = true;
                    player.jump_time = 0.0;
                }
            }
            KeyCode::LShift => {
                speed *= 1.7;
            }
            _ => {}
        }
    }

    if player.jumping {
        desired_movement.y = player.jump_power;
        player.jump_time += time.delta_seconds();

        if player.jump_time > player.jump_time_max {
            player.jumping = false;
        }
    }

    if let Some(&KinematicCharacterControllerOutput {
        grounded,
        effective_translation,
        ..
    }) = controller_output
    {
        player.grounded = grounded;

        if effective_translation.x != 0.0 || effective_translation.z != 0.0 {
            let angle = (-effective_translation.z).atan2(effective_translation.x);
            transform.rotation = transform
                .rotation
                .lerp(Quat::from_rotation_y(angle), player.rotate_speed);
        }
    }

    desired_movement *= speed;
    controller.translation = Some(desired_movement);
    player.current_speed = desired_movement;
}
