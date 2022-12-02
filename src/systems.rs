use crate::components::{Foot, FootAnchor, FootPole, FootTarget, MoveAnchorEvent, Player};
use bevy::{prelude::*, math::Vec3Swizzles};
use bevy_tweening::{lens::*, *};

// keeping foot on anchor
pub fn anchor_system(
    mut anchor_query: Query<(&FootAnchor, &mut Transform), Without<Foot>>,
    mut foot_query: Query<&mut Transform, (With<Foot>, Without<FootAnchor>)>,
    mut target_query: Query<&FootTarget>,
) {
    for (anchor, mut anchor_transform) in anchor_query.iter_mut() {
        if let Ok(mut foot_transform) = foot_query.get_mut(anchor.foot.unwrap()) {
            if !anchor.moving {
                foot_transform.translation = anchor_transform.translation;
            }
        }
    }
}

// trigger event
pub fn anchor_move_event_trigger_system(
    time: Res<Time>,
    foot_targets: Query<(Entity, &FootTarget, &GlobalTransform), Without<FootAnchor>>,
    mut anchor_query: Query<
        (Entity, &mut FootAnchor, &GlobalTransform),
        (With<FootAnchor>, Without<FootTarget>),
    >,
    mut move_event_writer: EventWriter<MoveAnchorEvent>,
) {
    for (target_entity, foot_target, target_transform) in foot_targets.iter() {
        if let Ok((anchor_entity, mut anchor, anchor_transform)) =
            anchor_query.get_mut(foot_target.anchor)
        {
            let distance = anchor_transform
                .translation().xz()
                .distance(target_transform.translation().xz()); //only xz distance
            // tick the timer
            anchor.animation_timer.tick(time.delta());

            if anchor.animation_timer.finished() {
                anchor.moving = false;
            }

            if anchor.moving == true {
                continue;
            }

            if distance > anchor.max_distance {
                move_event_writer.send(MoveAnchorEvent {
                    anchor: foot_target.anchor,
                    target: target_entity,
                });
                anchor.moving = true;
                // info!("Triggering foot {:?}", anchor.max_distance);
                // info!("foot.desired_pos: {:?}", target_transform.translation());
            }
        }
    }
}

// event handler
pub fn anchor_move_event_system(
    mut commands: Commands,
    mut reader: EventReader<MoveAnchorEvent>,
    mut anchor_query: Query<(&mut FootAnchor, &mut Transform), Without<FootTarget>>,
    mut target_query: Query<(&GlobalTransform, With<FootTarget>)>,
) {
    for event in reader.iter() {
        if let Ok((mut anchor, mut anchor_transform)) = anchor_query.get_mut(event.anchor) {
            if let Ok(target_transform) = target_query.get_mut(event.target) {

                let mut target_position = target_transform.0.translation().clone();
                 let ahead = target_position - anchor_transform.translation;
                if anchor.inverted {
                    target_position += Vec3::new(0.0, 0.6, 0.0);
                }

                target_position += ahead/2.0;
                anchor.inverted = !anchor.inverted;

                let tween = Tween::new(
                    EaseFunction::BounceInOut,
                    anchor.animation_duration,
                    TransformPositionLens2 {
                        start: anchor_transform.translation,
                        end: target_position,
                    },
                )
                .with_repeat_count(RepeatCount::Finite(1))
                .with_repeat_strategy(RepeatStrategy::MirroredRepeat);
                info!("Tweening foot {:?}", ahead);

                // commands
                //     .entity(event.anchor)
                //     .insert(anchor_transform.ease_to(
                //         Transform::from_translation(target_position),
                //         bevy_easings::EaseFunction::BounceInOut,
                //         EasingType::Once {
                //             duration: anchor.animation_duration,
                //         },
                //     ));

                commands.entity(event.anchor).insert(Animator::new(tween));
                anchor.animation_timer.reset();
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TransformPositionLens2 {
    /// Start value of the translation.
    pub start: Vec3,
    /// End value of the translation.
    pub end: Vec3,
}

impl Lens<Transform> for TransformPositionLens2 {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.translation = value;
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
