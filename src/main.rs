mod components;
mod ik_systems;
mod input_controller;
mod systems;

use components::{MoveAnchorEvent, Player};
use ik_systems::*;
use systems::*;

use bevy::{math::vec3, prelude::*};
use bevy_flycam::PlayerPlugin;
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_inverse_kinematics::InverseKinematicsPlugin;
use bevy_mod_wanderlust::{CharacterControllerBundle, ControllerPhysicsBundle};
use bevy_obj::*;
use bevy_rapier3d::prelude::*;
use bevy_tweening::{lens::*, *};
use input_controller::InputControllerPlugin;

fn main() {
    info!("main");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InfiniteGridPlugin)
        .add_plugin(InputControllerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InverseKinematicsPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(on_added_setup_ik)
        .add_system(target_system)
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
        transform: Transform::from_xyz(-12.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
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
    commands
        .spawn(SceneBundle {
            scene: assets.load("crab/crab_final.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert((
            Player {},
            CharacterControllerBundle {
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
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
        ));
}
