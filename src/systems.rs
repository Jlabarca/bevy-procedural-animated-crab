use crate::components::{Foot, FootAnchor, FootPole, FootTarget, MoveAnchorEvent, Player};
use bevy::prelude::*;
use bevy_easings::*;

// keeping foot on anchor
pub fn anchor_system(
    mut anchor_query: Query<(&FootAnchor, &mut Transform), Without<Foot>>,
    mut foot_query: Query<&mut Transform, (With<Foot>, Without<FootAnchor>)>,
) {
    for (anchor, mut anchor_transform) in anchor_query.iter_mut() {
        if let Ok(mut foot_transform) = foot_query.get_mut(anchor.foot.unwrap()) {
            if !anchor.moving {
                //foot_transform.translation = anchor_transform.translation.clone();
                //info!("anchor_system {:?}", foot_transform.translation);
                foot_transform.translation = anchor_transform.translation.clone();
            }
        }
    }
}

// trigger event
pub fn anchor_move_event_trigger_system(
    time: Res<Time>,
    foot_targets: Query<(&FootTarget, &GlobalTransform), Without<FootAnchor>>,
    mut anchor_query: Query<
        (&mut FootAnchor, &GlobalTransform),
        (With<FootAnchor>, Without<FootTarget>),
    >,
    mut move_event_writer: EventWriter<MoveAnchorEvent>,
) {
    for (foot_target, target_transform) in foot_targets.iter() {
        if let Ok((mut anchor, anchor_transform)) = anchor_query.get_mut(foot_target.anchor) {
            let distance = anchor_transform
                .translation_vec3a()
                .distance(target_transform.translation_vec3a());
            // tick the timer
            anchor.animation_timer.tick(time.delta());

            if anchor.animation_timer.finished() {
                anchor.moving = false;
            }

            if anchor.moving == true {
                continue;
            }

            if distance > anchor.max_distance {
                anchor.desired_pos = target_transform.translation();
                move_event_writer.send(MoveAnchorEvent {
                    anchor: foot_target.anchor,
                    target: anchor.desired_pos,
                });
                anchor.moving = true;

                info!("Triggering foot {:?}", anchor.max_distance);
                info!("foot.desired_pos: {:?}", anchor.desired_pos);
            }
        }
    }
}

// event handler
pub fn anchor_move_event_system(
    mut commands: Commands,
    mut reader: EventReader<MoveAnchorEvent>,
    mut anchor_query: Query<(&mut FootAnchor, &Transform)>,
) {
    for event in reader.iter() {
        info!("anchor_move_event_system entity {:?}", event.anchor);

        if let Ok((mut anchor, anchor_transform)) = anchor_query.get_mut(event.anchor) {
            info!("ease_to foot {:?}", anchor_transform);
            info!("event.target {:?}", event.target);
            commands
                .entity(event.anchor)
                .insert(anchor_transform.ease_to(
                    Transform::from_translation(anchor.desired_pos.clone()),
                    bevy_easings::EaseFunction::CircularInOut,
                    EasingType::Once {
                        duration: anchor.animation_duration,
                    },
                ));

            anchor.animation_timer.reset();
            info!("event.target: {:?}", event.target);
        }
    }
}

// foot target at body side
pub fn target_system(mut foot_targets: Query<(&FootTarget, &mut Transform), Without<Player>>) {
    for (foot_target, mut target_transform) in foot_targets.iter_mut() {
        //let pos = player_query.get(foot_target.owner).unwrap().1.translation;
        target_transform.translation.x = foot_target.pos_offset.x;
        target_transform.translation.z = foot_target.pos_offset.z;
        //target_transform.translation.y = 1.0;
    }

    //Calculate height using raycast
    // for (foot_target, mut transform) in foot_targets.iter_mut() {
    //     let pos = target_query.get(foot_target.owner).unwrap().1.translation;
    //     let mut ray = Ray::new(
    //         Vec3::new(pos.x, pos.y + 10.0, pos.z),
    //         Vec3::new(0.0, -1.0, 0.0),
    //     );
    //     let mut hit = false;
    //     let mut hit_pos = Vec3::new(0.0, 0.0, 0.0);
    //     for result in foot_target.owner.world().raycast(ray) {
    //         if result.entity != foot_target.owner {
    //             hit = true;
    //             hit_pos = result.position;
    //             break;
    //         }
    //     }
    //     if hit {
    //         transform.translation.y = hit_pos.y + 0.1;
    //     }
    // }
}

pub fn pole_system(
    mut foot_poles: Query<(&FootPole, &mut Transform), Without<Player>>,
    player_query: Query<(&Player, &Transform), Without<FootPole>>,
) {
    for (foot_pole, mut target_transform) in foot_poles.iter_mut() {
        let player_pos = player_query.get(foot_pole.owner).unwrap().1.translation;
        target_transform.translation = player_pos + foot_pole.pos_offset;
    }

}