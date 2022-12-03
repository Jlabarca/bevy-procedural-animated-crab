mod components;
mod ik_systems;
mod systems;

use components::{MoveAnchorEvent, Player};
use ik_systems::*;
use rand::Rng;
use systems::*;

use bevy::{math::vec3, prelude::*};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_inverse_kinematics::InverseKinematicsPlugin;
use bevy_obj::*;
use bevy_rapier3d::prelude::*;
use bevy_tweening::{lens::*, *};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        //.add_plugin(PlayerPlugin)
        .add_plugin(InfiniteGridPlugin)
        //.add_plugin(InputControllerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InverseKinematicsPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(on_added_setup_ik)
        .add_system(target_system)
        .add_system(handle_move)
        // .add_system(anchor_system)
        // .add_system(pole_system)
        .add_event::<MoveAnchorEvent>()
        .add_system(anchor_move_event_trigger_system)
        .add_system(anchor_move_event_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // Camera
    commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(7.5, 10.5, 7.5)
                    .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
                projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                    fov: std::f32::consts::FRAC_PI_4,
                    aspect_ratio: 1.0,
                    near: 0.1,
                    far: 100.0,
                }),
                ..default()
            });
        });

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
        transform: Transform::from_xyz(12.0, 18.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    //floor
    let ground_size = 200.1;
    let ground_height = 0.1;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size })),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        },
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    // grid
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    });

    // player
    commands
        .spawn(SceneBundle {
            scene: assets.load("crab/crab_final.gltf#Scene0"),
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 180.0_f32.to_radians())),
            ..default()
        })
        .insert((
            Player {
                move_speed: 0.05,
                rotate_speed: 0.1,
                grounded: false,
                jumping: false,
                jump_power: 2.0,
                jump_time: 0.0,
                jump_time_max: 0.2,
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule(
                vec3(0.0, 1.5 - 0.25, 0.0),
                vec3(0.0, 1.5 + 0.25, 0.0),
                1.5,
            ),
            GravityScale(1.0),
            Ccd::enabled(),
            KinematicCharacterController {
                translation: Some(Vec3::new(0.0, 1.75, 0.0)),
                // The character offset is set to 0.01.
                offset: CharacterLength::Absolute(0.125),
                max_slope_climb_angle: 45.0_f32.to_radians(),
                // Automatically slide down on slopes smaller than 30 degrees.
                min_slope_slide_angle: 30.0_f32.to_radians(),
                ..default()
            },
        ));

    // boxes
    let mut rng = rand::thread_rng();

    let num = 4;
    //todo: add randomness
    for j in 0usize..10 {
        let rad = rng.gen_range(0.2..0.5);
        let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

        for i in 0..num {
            for k in 0usize..num {
                let shift = rad * 2.0 + rad;
                let centerx = shift * (num / 2) as f32;
                let centery = shift / 2.0;
                let centerz = shift * (num / 2) as f32;


                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;

                commands
                    .spawn((
                        //TransformBundle::from(Transform::from_rotation(Quat::from_rotation_x(0.2))),
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: rad * 2.0})),
                            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                            transform: Transform::from_xyz(x, y, z),
                            ..default()
                        },
                        AdditionalMassProperties::Mass(3.0),
                        RigidBody::Dynamic,
                        Collider::cuboid(rad, rad, rad),
                    ));
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
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
    let x = Vec3::new(1.0, 0.0, -1.0);
    let z = Vec3::new(1.0, 0.0, 1.0);

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
                speed /= 10.0;
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
}
