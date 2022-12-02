use crate::components::{Foot, FootAnchor, FootPole, FootTarget, Player};
use bevy::prelude::*;
use bevy_mod_inverse_kinematics::IkConstraint;

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
    for added_entity in added_player_query.iter() {
        info!("setup_ik_on_added: {}", names.get(added_entity).unwrap()); //wtf silent error todo: check wtf


        let right_hand = find_entity(
            &EntityPath {
                parts: vec![
                    "L.Shoulder.001".into(),
                    "L.Leg.001".into(),
                    "L.Foot.001".into(),
                    "L.Toe.001".into(),
                ],
            },
            added_entity,
            &children,
            &names,
        )
        .unwrap();

        generate_leg_kinematics(
            player_entity,
            right_hand,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(1.1, 0.6, 0.5),
            3,
            30
        );

        let right_hand2 = find_entity(
            &EntityPath {
                parts: vec![
                    "L.Shoulder.002".into(),
                    "L.Leg.002".into(),
                    "L.Foot.002".into(),
                    "L.Toe.002".into(),
                ],
            },
            added_entity,
            &children,
            &names,
        )
        .unwrap();

        generate_leg_kinematics(
            player_entity,
            right_hand2,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(1.1, 0.6, 0.0),
            2,
            20
        );

        let right_hand3 = find_entity(
            &EntityPath {
                parts: vec![
                    "L.Shoulder.003".into(),
                    "L.Leg.003".into(),
                    "L.Foot.003".into(),
                    "L.Toe.003".into(),
                ],
            },
            added_entity,
            &children,
            &names,
        )
        .unwrap();

        generate_leg_kinematics(
            player_entity,
            right_hand3,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(1.1, 0.6, -0.5),
            2,
            20
        );

        let right_handR = find_entity(
            &EntityPath {
                parts: vec![
                    "R.Shoulder.001".into(),
                    "R.Leg.001".into(),
                    "R.Foot.001".into(),
                    "R.Toe.001".into(),
                ],
            },
            added_entity,
            &children,
            &names,
        )
        .unwrap();

        generate_leg_kinematics(
            player_entity,
            right_handR,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(-1.1, 0.6, 0.5),
            3,
            30
        );

        let right_hand2R = find_entity(
            &EntityPath {
                parts: vec![
                    "R.Shoulder.002".into(),
                    "R.Leg.002".into(),
                    "R.Foot.002".into(),
                    "R.Toe.002".into(),
                ],
            },
            added_entity,
            &children,
            &names,
        )
        .unwrap();

        generate_leg_kinematics(
            player_entity,
            right_hand2R,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(-1.1, 0.6, 0.0),
            2,
            20
        );

        let right_hand3R = find_entity(
            &EntityPath {
                parts: vec![
                    "R.Shoulder.003".into(),
                    "R.Leg.003".into(),
                    "R.Foot.003".into(),
                    "R.Toe.003".into(),
                ],
            },
            added_entity,
            &children,
            &names,
        )
        .unwrap();

        generate_leg_kinematics(
            player_entity,
            right_hand3R,
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3::new(-1.1, 0.6, -0.5),
            2,
            20
        );
    }
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
) {
    commands.entity(foot_entity).insert(Foot {});

    let pole = commands
        .spawn((
            PbrBundle {
                transform: Transform::from_translation(offset_spread + Vec3::new(1.0, 1.0, 1.0)) ,
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
                pos_offset: offset_spread + Vec3::new(0.0, 1.0, 0.0),
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
                foot: Some(foot_entity),
                ..default()
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

    commands
        .entity(player_entity)
        //.add_child(anchor)
        .add_child(target)
        .add_child(pole);
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
