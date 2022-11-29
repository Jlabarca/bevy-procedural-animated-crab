mod input_controller;

use std::time::Duration;

use bevy::{math::vec3, prelude::*, log::LogPlugin};
use bevy_easings::*;
use bevy_flycam::PlayerPlugin;
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_inverse_kinematics::{IkConstraint, InverseKinematicsPlugin};
use bevy_mod_wanderlust::{CharacterControllerBundle, ControllerPhysicsBundle};
use bevy_obj::*;
use bevy_rapier3d::prelude::*;
use input_controller::InputControllerPlugin;

fn main() {
    info!("main");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .add_plugin(PlayerPlugin) //камера
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(InputControllerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InverseKinematicsPlugin)
        .add_plugin(EasingsPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(setup_ik)
        //.add_system(manually_target)
        .add_system(foots_system)
        .add_system(foot_target_system)
        .add_system(foot_move_system)
        .add_event::<MoveFootEvent>()
        .run();
}

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Foot {
    current_pos: Vec3,
    desired_pos: Vec3,
    pos_error_margin: f32,
    max_distance: f32,
    moving: bool,
}

pub struct MoveFootEvent {
    foot: Entity,
    target: Vec3,
}

impl Default for Foot {
    fn default() -> Self {
        Self {
            current_pos: Vec3::ZERO,
            desired_pos: Vec3::ZERO,
            pos_error_margin: 0.025,
            max_distance: 0.5,
            moving: false,
        }
    }
}

#[derive(Component)]
struct FootTarget {
    owner: Entity,
    foot: Entity,
    pos_offset: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
        info!("setup");

    // commands
    //     .spawn(SpatialBundle::default())
    //     .with_children(|parent| {
    //         parent.spawn(Camera3dBundle {
    //             transform: Transform::from_xyz(-0.5, 1.5, 2.5)
    //                 .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    //             projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
    //                 fov: std::f32::consts::FRAC_PI_4,
    //                 aspect_ratio: 1.0,
    //                 near: 0.1,
    //                 far: 100.0,
    //             }),
    //             ..default()
    //         });
    //     });

    let size = 30.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -size,
                right: size,
                bottom: -size,
                top: size,
                near: -size,
                far: size,
                ..default()
            },
            ..default()
        },
        transform: Transform::from_xyz(-8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    //floor
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        ..default()
    });

    // grid
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    });

    //model
    let parent_entity = commands
        .spawn(SceneBundle {
            scene: assets.load("skin.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert((
            Player {},
            CharacterControllerBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                physics: ControllerPhysicsBundle {
                    rigidbody: RigidBody::Dynamic,
                    locked_axes: LockedAxes::ROTATION_LOCKED,
                    collider: Collider::capsule(
                        vec3(0.0, 2. - 0.5, 0.0),
                        vec3(0.0, 2. + 0.5, 0.0),
                        1.,
                    ),
                    gravity: GravityScale(0.0),
                    ccd: Ccd::enabled(),
                    ..default()
                },
                ..default()
            },
        ))
        .id();
}

fn setup_ik(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    added_query: Query<(Entity, &Parent), Added<AnimationPlayer>>,
    children: Query<&Children>,
    names: Query<&Name>,
) {

    // Use the presence of `AnimationPlayer` to determine the root entity of the skeleton.
    for (entity, _parent) in added_query.iter() {
        // Try to get the entity for the right hand joint.
        info!("setup_ik");

        let right_hand = find_entity(
            &EntityPath {
                parts: vec![
                    "Pelvis".into(),
                    "Spine1".into(),
                    "Spine2".into(),
                    "Collar.R".into(),
                    "UpperArm.R".into(),
                    "ForeArm.R".into(),
                    "Hand.R".into(),
                ],
            },
            entity,
            &children,
            &names,
        )
        .unwrap();

        //pelota verde
        let pole_target = commands
            .spawn(PbrBundle {
                transform: Transform::from_xyz(-1.0, 0.4, -0.2),
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.05,
                    subdivisions: 1,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                ..default()
            })
            .id();

        //Target red ball
        let target = commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_xyz(0.3, 0.8, 0.2),
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.25,
                        subdivisions: 1,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::RED,
                        ..default()
                    }),
                    ..default()
                },
                FootTarget {
                    owner: entity,
                    foot: right_hand,
                    pos_offset: Vec3::new(2., 0.0, 2.),
                },
            ))
            .id();
        info!("red ball: {:?}", target);
        // Add an IK constraint to the right hand, using the targets that were created earlier.
        commands.entity(right_hand).insert(IkConstraint {
            chain_length: 2,
            iterations: 20,
            target,
            pole_target: Some(pole_target),
            pole_angle: -std::f32::consts::FRAC_PI_2,
        });
    }
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
                if let Ok(name) = names.get(*child) {
                    if name == part {
                        // Found a children with the right name, continue to the next part
                        current_entity = *child;
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            warn!("Entity not found for path {:?} on part {:?}", path, part);
            return Err(());
        }
    }

    Ok(current_entity)
}

#[derive(Component)]
pub struct ManuallyTarget(Vec4);

fn foots_system(
    foot_targets: Query<(&FootTarget, &Transform)>,
    mut target_query: Query<(&mut Foot, &Transform)>,
    mut move_event_writer: EventWriter<MoveFootEvent>,
) {
    for (foot_target, target_transform) in foot_targets.iter() {
        let (mut foot, foot_transform) = target_query.get_mut(foot_target.foot).unwrap();

        //if foot_target distance is greater than
        let distance = foot_transform
            .translation
            .distance(target_transform.translation);
        if !foot.moving && distance > foot.max_distance {
            move_event_writer.send(MoveFootEvent {
                foot: foot_target.foot,
                target: target_transform.translation.clone(),
            });
            foot.moving = true;
            info!("moving foot");
        } else if foot.moving && distance.abs() < foot.pos_error_margin {
            foot.moving = false;
            info!("stopping foot");
        }
    }
}

fn foot_move_system(
    mut reader: EventReader<MoveFootEvent>,
    mut query: Query<(&mut Foot, &Transform)>,
) {
    for event in reader.iter() {
        if let Ok((mut foot, foot_transform)) = query.get_mut(event.foot) {
            foot_transform.ease_to(
                Transform::from_translation(event.target),
                bevy_easings::EaseFunction::SineIn,
                EasingType::Once {
                    duration: Duration::from_secs_f32(0.33),
                },
            );
            foot.moving = true;
        }
    }
}

fn foot_target_system(
    mut foot_targets: Query<(&FootTarget, &mut Transform), Without<Player>>,
    player_query: Query<(&Player, &Transform), Without<FootTarget>>,
) {
    //let (player, player_transform) = player_query.single();

    //Keep it at body side
    for (foot_target, mut transform) in foot_targets.iter_mut() {
        let pos = player_query.get(foot_target.owner).unwrap().1.translation;
        transform.translation.x = pos.x + foot_target.pos_offset.x;
        transform.translation.z = pos.z + foot_target.pos_offset.z;
        transform.translation.y = 0.5;
        info!("{:?}", transform.translation);
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
