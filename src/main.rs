mod components;
mod ik_systems;
mod systems;

use bevy_flycam::PlayerPlugin;
use components::{Ground, MoveAnchorEvent, Player};
use ik_systems::*;
use rand::Rng;
use systems::*;

use bevy::{math::vec3, prelude::*};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_inverse_kinematics::InverseKinematicsPlugin;
use bevy_obj::*;
use bevy_rapier3d::{na::Matrix4, prelude::*};
use bevy_tweening::{lens::*, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InverseKinematicsPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(on_added_setup_ik)
        .add_system(target_at_side_system)
        .add_system(target_height_system)
        .add_system(handle_move)
        // .add_system(force_foot_on_anchor_system)
        .add_system(pole_system)
        .add_event::<MoveAnchorEvent>()
        .add_system(anchor_move_event_trigger_system)
        .add_system(anchor_move_event_system)
        .register_type::<Player>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    // Camera
    // commands
    //     .spawn(SpatialBundle::default())
    //     .with_children(|parent| {
    //         parent.spawn(Camera3dBundle {
    //             transform: Transform::from_xyz(0.0, 10.5, 7.5)
    //                 .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    //             // projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
    //             //     fov: std::f32::consts::FRAC_PI_4,
    //             //     aspect_ratio: 1.0,
    //             //     near: 0.1,
    //             //     far: 100.0,
    //             // }),
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
        transform: Transform::from_xyz(12.0, 18.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ground
    let ground_size = 200.1;
    let ground_height = 0.01;

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GRAY,
                    ..default()
                }),
                ..default()
            },
            Collider::cuboid(ground_size, ground_height, ground_size),
            Ground {},
        ))
        .insert(Visibility { is_visible: true });

    // grid
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    });

    // player
    let crab_model = commands
        .spawn(SceneBundle {
            scene: assets.load("crab/crab.gltf#Scene0"),
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::ONE,
                Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 180.0_f32.to_radians()), //model looks to positive Z by default
                Vec3::new(0.0, -1.3, 0.0), //todo: set this dynamic so when collider step up, model step up too but lerping
            )),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .id();

    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(0.0, 2.0, 0.0)),
            VisibilityBundle::default(),
            Player {
                current_speed: Vec3::ZERO,
                move_speed: 0.04,
                rotate_speed: 0.00,
                grounded: false,
                jumping: false,
                jump_power: 2.0,
                jump_time: 0.0,
                jump_time_max: 0.3,
                walk_height: 0.5,
                walk_width: 0.5,
                walk_spread: Vec3::new(1.1, 1.0, 1.3),//hardcoded for crab
                pole_offset: Vec3::ZERO,
                pole_spread: Vec3::ONE,
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::round_cylinder(1.3, 0.35, 0.20), //todo: fix changing collider size affect crab_model translation
            GravityScale(1.0),
            Ccd::enabled(),
            KinematicCharacterController {
                //translation: Some(Vec3::new(0.0, -2.1, 0.0)),
                offset: CharacterLength::Absolute(0.05),
                max_slope_climb_angle: 70.0_f32.to_radians(), //slope would make autostep not trigger if the climb angle is too high
                min_slope_slide_angle: 30.0_f32.to_radians(),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(1.25),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: true,
                }),
                ..default()
            },
        ))
        // .insert(Visibility {
        //     is_visible: false,
        // })
        .add_child(crab_model);

    spawn_boxes(commands, meshes, materials, 10);
}

fn spawn_boxes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    size: usize,
) {
    let mut rng = rand::thread_rng();

    let num = 4;
    let mut rad: f32 = 0.5;
    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
    //todo: add randomness
    for j in 0usize..size {
        for i in 0..num {
            for k in 0usize..num {
                rad = rng.gen_range(0.1..=0.2);
                offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
                let shift = rad * 2.0 + rad;
                let centerx = shift * (num / 2) as f32;
                let centery = shift / 2.0;
                let centerz = shift * (num / 2) as f32;

                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;

                commands.spawn((
                    //TransformBundle::from(Transform::from_rotation(Quat::from_rotation_x(0.2))),
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: rad * 2.0 })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        transform: Transform::from_xyz(x, y, z),
                        ..default()
                    },
                    Name::new(format!("Box_{}_{}_{}", i, j, k)),
                    AdditionalMassProperties::Mass(300.0),
                    RigidBody::Dynamic,
                    Collider::cuboid(rad, rad, rad),
                ));
            }
        }
        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}
