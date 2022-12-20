use std::time::Duration;

use crate::components::{Foot, FootAnchor, FootPole, FootTarget, Player};
use bevy::prelude::*;
use bevy_mod_inverse_kinematics::IkConstraint;
struct KinematicLeg {
    body_entity: Entity,
    legs_spread: Vec<f32>, // general - no deberia ir aca
    offset_spread: Vec<f32>,
    chain_length: usize, // no recuerdo que era
    joints_amount: usize,
}

pub fn on_added_setup_ik(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    added_player_query: Query<Entity, Added<AnimationPlayer>>,
    //children: Query<(&AnimationPlayer, &Children)>,
    children: Query<&Children>,
    names: Query<&Name>,
    player_query: Query<(Entity, With<Player>)>,
) {
    let player_entity = player_query.get_single().unwrap().0;
    let spread = Vec3::new(1.5, -1.0, 1.33);
    let chain_length = 2;
    let joints_amount = 4;

    for added_entity in added_player_query.iter() {
        generate_leg_kinematics(
            player_entity,
            find_entity(
                &EntityPath {
                    parts: get_parts("L", 1, joints_amount),
                },
                added_entity,
                &children,
                &names,
            )
            .unwrap(),
            &mut commands,
            &mut meshes,
            &mut materials,
            spread * Vec3::new(-0.6, 0.01, -0.33),
            chain_length,
            20,
            0.14,
            false,
        );

        generate_leg_kinematics(
            player_entity,
            find_entity(
                &EntityPath {
                    parts: get_parts("L", 2, joints_amount),
                },
                added_entity,
                &children,
                &names,
            )
            .unwrap(),
            &mut commands,
            &mut meshes,
            &mut materials,
            spread * Vec3::new(-0.8, 0.01, 0.0),
            chain_length,
            20,
            0.25,
            true,
        );

        generate_leg_kinematics(
            player_entity,
            find_entity(
                &EntityPath {
                    parts: get_parts("L", 3, joints_amount),
                },
                added_entity,
                &children,
                &names,
            )
            .unwrap(),
            &mut commands,
            &mut meshes,
            &mut materials,
            spread * Vec3::new(-0.5, 0.01, 0.6),
            chain_length,
            20,
            0.20,
            false,
        );

        generate_leg_kinematics(
            player_entity,
            find_entity(
                &EntityPath {
                    parts: get_parts("R", 1, joints_amount),
                },
                added_entity,
                &children,
                &names,
            )
            .unwrap(),
            &mut commands,
            &mut meshes,
            &mut materials,
            spread * Vec3::new(0.6, 0.01, -0.3),
            chain_length,
            20,
            0.25,
            true,
        );

        generate_leg_kinematics(
            player_entity,
            find_entity(
                &EntityPath {
                    parts: get_parts("R", 2, joints_amount),
                },
                added_entity,
                &children,
                &names,
            )
            .unwrap(),
            &mut commands,
            &mut meshes,
            &mut materials,
            spread * Vec3::new(0.7, 0.01, 0.2),
            chain_length,
            20,
            0.20,
            false,
        );

        generate_leg_kinematics(
            player_entity,
            find_entity(
                &EntityPath {
                    parts: get_parts("R", 3, joints_amount),
                },
                added_entity,
                &children,
                &names,
            )
            .unwrap(),
            &mut commands,
            &mut meshes,
            &mut materials,
            spread * Vec3::new(0.5, 0.01, 0.6),
            chain_length,
            20,
            0.25,
            true,
        );
    }
}

fn get_parts(prefix: &str, number: i32, size: usize) -> Vec<Name> {
    let a = vec![
        format!("{}.Shoulder.00{}", prefix, number).into(),
        format!("{}.Leg.00{}", prefix, number).into(),
        format!("{}.Foot.00{}", prefix, number).into(),
        format!("{}.Toe.00{}", prefix, number).into(),
    ];
    a[0..size].to_vec()
}

fn generate_leg_kinematics(
    player_entity: Entity,
    foot_entity: Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    offset_spread: Vec3,
    chain_length: usize,
    iterations: usize,
    distance: f32,
    inverted: bool,
) {
    commands.entity(foot_entity).insert(Foot {});
    let pole = commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(
                    offset_spread * Vec3::new(3.5, 1.0, 3.0) + Vec3::new(0.0, 2.0, 0.0),
                ),
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.05,
                    subdivisions: 1,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::YELLOW,
                    ..default()
                }),
                ..default()
            },
            FootPole {
                owner: player_entity,
                pos_offset: offset_spread * Vec3::new(3.5, 1.0, 3.0) + Vec3::new(0.0, 2.0, 0.0), //todo: remove?
            },
        ))
        .id();

    let anchor = commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(offset_spread),
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.05,
                    subdivisions: 1,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                ..default()
            },
            FootAnchor {
                owner: player_entity,
                foot: Some(foot_entity),
                target: None,
                animation_duration: Duration::from_secs_f32(distance * 1.2),
                animation_timer: Timer::new(Duration::from_secs_f32(distance * 1.2), TimerMode::Once),
                pos_error_margin: 0.2,
                max_distance: distance,
                moving: false,
                inverted,
            },
        ))
        .id();

    let target = commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(offset_spread),
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.05,
                    subdivisions: 1,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                ..default()
            },
            FootTarget {
                owner: player_entity,
                foot: foot_entity,
                anchor: anchor,
                pos_offset: offset_spread,
            },
        ))
        .id();

    commands.entity(foot_entity).insert(IkConstraint {
        chain_length,
        iterations,
        target: anchor,
        pole_target: Some(pole),
        pole_angle: -std::f32::consts::FRAC_PI_2,
    });

    // todo:adding leg as body child breaks everything rn, because it uses transform instead of global transform in some parts
    //.entity(player_entity)
    // .add_child(anchor)
    // .add_child(target)
    // .add_child(pole);
}

fn find_entity(
    path: &EntityPath,
    root: Entity,
    children: &Query<&Children>,
    names: &Query<&Name>,
) -> Result<Entity, ()> {
    let mut current_entity = root;

    for part in path.parts.iter() {
        let mut found = false;
        if let Ok(children) = children.get(current_entity) {
            for child in children.iter() {
                match part.as_str() {
                    _ => {
                        if let Ok(name) = names.get(*child) {
                            if name == part {
                                current_entity = *child;
                                found = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        if !found {
            warn!("Entity not found for path {:?} on part {:?}", path, part); // not happening
            return Err(());
        }
    }

    Ok(current_entity)
}
