mod input_controller;
use bevy::{math::vec3, prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_wanderlust::{CharacterControllerBundle, ControllerPhysicsBundle};
use bevy_rapier3d::prelude::*;
use input_controller::InputControllerPlugin;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin, InfiniteGrid};

#[derive(Component, Default, Reflect)]
struct Player {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InputControllerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_startup_system(handle_procedural_animation)
        .add_system(spider_walk_system)
        .add_system(print_ball_altitude)
        .run();
}

fn handle_procedural_animation(
    mut commands: Commands
) {
    create_prismatic_joints(&mut commands, Vec3::new(0.0, 0.0, 0.0), 5);
}
fn create_prismatic_joints(
    commands: &mut Commands,
    origin: Vect,
    num: usize,
) {

}


fn setup_graphics(mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 40.0,
            scaling_mode: ScalingMode::FixedVertical(1.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(100.0, 100.0, 100.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0).with_rotation(Quat::from_euler(
            EulerRot::XYZ,
            0.5,
            -2.6,
            1.0,
        )),
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
}

const DEFAULT_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const OBJECT_COLOR: Color = Color::rgb(0.3, 0.5, 0.3);

fn setup_physics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground */
    let cube_width = 100.0;
    let cube_position = Transform::from_xyz(0.0, -2.0, 0.0);
    let cube_mesh_handle = meshes.add(Mesh::from(shape::Box::new(cube_width, 0.5, cube_width)));
    let texture_handle = asset_server.load("resources/tile.png");


    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        //unlit: true,
        ..default()
    });

    commands.spawn((
        Collider::cuboid(cube_width, 0.5, cube_width),
        PbrBundle {
            mesh: cube_mesh_handle.clone(),
            material: material_handle,
            transform: Transform {
                translation: cube_position.translation,
                //rotation: Quat::from_rotation_x(-1.5708),
                ..default()
            },
            ..default()
        },
    ));

    spawn_spider(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 4.0, 0.0),
        1.5,
    );

    spawn_ball(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 4.0, 0.0),
        1.5,
    );

    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 6.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        OBJECT_COLOR,
    );

    //spawn_cubes(&mut commands, &mut meshes, &mut materials);
}

fn print_ball_altitude(positions: Query<&Transform, With<Player>>) {
    for transform in positions.iter() {
        println!("Player altitude: {}", transform.translation.y);
    }
}

fn spider_walk_system(positions: Query<&Transform, With<RigidBody>>) {
    // for transform in positions.iter() {
    //     println!("Ball altitude: {}", transform.translation.y);
    // }
}

fn spawn_cubes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let num = 8;
    let rad = 1.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
    let mut color = 0;
    let colors = [
        Color::hsl(220.0, 1.0, 0.3),
        Color::hsl(180.0, 1.0, 0.3),
        Color::hsl(260.0, 1.0, 0.7),
    ];

    for j in 0usize..20 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;
                color += 1;

                spawn_cube(
                    commands,
                    meshes,
                    materials,
                    Vec3::new(x, y, z),
                    Vec3::new(1.0, 1.0, 1.0),
                    colors[color % 3],
                );
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}

fn spawn_spider(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    radius: f32,
) {
    //araña body
    let mut parent_entity = commands
        .spawn(CharacterControllerBundle {
            physics: ControllerPhysicsBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::capsule(vec3(0.0, 0.0, 0.0), vec3(0.0, 1., 0.0), 1.),
                velocity: default(),
                gravity: GravityScale(0.0),
                friction: Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                damping: Damping {
                    linear_damping: 0.0,
                    angular_damping: 0.0,
                },
                restitution: Restitution {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
                read_mass_properties: default(),
                ..default()
            },
            ..default()
        })
        .insert(Player::default())
        .id();

    //shoulder

    let rad = 0.4;
    let dz = 0.0;

    let axis = Vec3::new(1.0, 1.0, 0.0);

    let prism = PrismaticJointBuilder::new(axis)
        .local_anchor2(Vec3::new(0.0, 1.0, 0.0))
        .limits([-2.0, 2.0]);
    let joint = ImpulseJoint::new(parent_entity, prism);

    parent_entity = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z + dz)),
            RigidBody::Dynamic,
            Collider::cuboid(rad, rad, rad),
            joint,
        ))
        .id();

    //knee
    let rad = 0.4;
    let dz = -2.0;

    let axis = Vec3::new(0.0, 1.0, 0.0);

    let prism = PrismaticJointBuilder::new(axis)
        //.local_axis1(Vec3::new(0.0, 1.0, 0.0))
        .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
        .local_anchor2(Vec3::new(1.0, -1.0, 0.0));
    let joint = ImpulseJoint::new(parent_entity, prism);

    parent_entity = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z + dz)),
            RigidBody::Dynamic,
            Collider::cuboid(rad, rad, rad),
            joint,
        ))
        .id();

    //foot
    let rad = 0.4;
    let dz = -2.0;

    let axis = Vec3::new(1.0, 1.0, 1.0);

    let prism = PrismaticJointBuilder::new(axis)
        //.local_axis1(Vec3::new(0.0, 1.0, 0.0))
        .local_anchor1(Vec3::new(1.0, 1.0, 0.0));
    let joint = ImpulseJoint::new(parent_entity, prism);

    parent_entity = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z + dz)),
            RigidBody::KinematicPositionBased,
            Collider::cuboid(rad, rad, rad),
            joint,
        ))
        .id();

    // let dz = (2 + 1) as f32 * shift;

    // let axis = if 2 % 2 == 0 {
    //     Vec3::new(1.0, 1.0, 0.0)
    // } else {
    //     Vec3::new(-1.0, 1.0, 0.0)
    // };

    // let prism = PrismaticJointBuilder::new(axis)
    //     .local_anchor2(Vec3::new(0.0, 0.0, -shift))
    //     .limits([-2.0, 2.0]);
    // let joint = ImpulseJoint::new(entity, prism);
    // entity = commands
    //     .spawn((
    //         TransformBundle::from(Transform::from_xyz(position.x, position.y, position.z + dz)),
    //         RigidBody::Fixed,
    //         Collider::cuboid(rad, rad, rad),
    //         joint,
    //     ))
    //     .id();

    //         .insert((PbrBundle {
    //             mesh: meshes.add(Mesh::from(shape::Icosphere {
    //                 radius: radius,
    //                 subdivisions: 32,
    //             })),
    //             material: materials.add(StandardMaterial {
    //                 base_color: Color::hex("ffd891").unwrap(),
    //                 // vary key PBR parameters on a grid of spheres to show the effect
    //                 // metallic: y01,
    //                 // perceptual_roughness: x01,
    //                 ..default()
    //             }),
    //             transform: Transform::from(Transform::from_translation(position)),
    //             ..default()
    //         }));
}

fn spawn_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    radius: f32,
) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::ball(radius),
        Restitution::coefficient(0.7),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: radius,
                subdivisions: 32,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("ffd891").unwrap(),
                // vary key PBR parameters on a grid of spheres to show the effect
                // metallic: y01,
                // perceptual_roughness: x01,
                ..default()
            }),
            transform: Transform::from(Transform::from_translation(position)),
            ..default()
        },
    ));
}

fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    size: Vec3,
    color: Color,
) {
    let cube_mesh_handle = meshes.add(Mesh::from(shape::Box::new(size.x, size.y, size.z)));

    commands
        .spawn(RigidBody::Dynamic)
        //.insert(Transform::from_xyz(x, y, z))
        //.insert(GlobalTransform::default())
        .insert(Collider::cuboid(size.x, size.y, size.z))
        .insert(ColliderDebugColor(color))
        .insert(PbrBundle {
            mesh: cube_mesh_handle.clone(),
            material: materials.add(color.into()),
            transform: Transform::from_translation(position),
            ..default()
        });
}
