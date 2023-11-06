use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_xpbd_3d::prelude::*;

use rand::Rng;
use std::f32::consts::PI;

const PLAYER_SPEED: f32 = 5.;
const PLAYER_GRAVITY: f32 = -9.81;
const PLAYRE_JUMP_HEIGHT: f32 = 1.;

const CAMERA_TRANSLATION: Vec3 = Vec3::new(0.0, 15.0, 15.0);

#[derive(Component)]
struct Player {
    velocity: Vec3,
}

fn main() {
    // Ambient light
    let ambient_light: AmbientLight = AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    };

    // App
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(ambient_light)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, player_gravity)
        .add_systems(Update, camera_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_translation: Vec3 = Vec3::new(0., 10., 0.);

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(CAMERA_TRANSLATION)
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });

    // Plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(25.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(25.0, 0.002, 25.0),
    ));

    // Cube
    let cube_translations: [Vec3; 4] = [
        Vec3::new(5., 4.5, 0.),
        Vec3::new(-5., 4.5, 0.),
        Vec3::new(0., 4.5, 5.),
        Vec3::new(0., 4.5, -5.),
    ];

    for mut cube_translation in cube_translations {
        let cube_translation_y = rand::thread_rng().gen_range(1..=10) as f32;

        cube_translation.y = cube_translation_y;

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::Cube::default().into()),
                material: materials.add(Color::rgb(0., 0., 0.).into()),
                transform: Transform::from_translation(cube_translation),
                ..default()
            },
            RigidBody::Dynamic,
            AngularVelocity(Vec3::new(2.5, 3.4, 1.6)),
            Collider::cuboid(1.0, 1.0, 1.0),
        ));
    }

    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Capsule::default().into()),
            material: materials.add(Color::rgb(0., 0., 0.).into()),
            transform: Transform::from_translation(player_translation),
            ..default()
        },
        RigidBody::Kinematic,
        Collider::capsule(2., 0.5),
        Player {
            velocity: Vec3::ZERO,
        },
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn camera_movement(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let player_transform = player_query.get_single().unwrap();
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    camera_transform.translation = player_transform.translation + CAMERA_TRANSLATION;
}

fn player_gravity(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let (mut player_transform, mut player) = query.get_single_mut().unwrap();

    let grounded_player: bool = player_transform.translation.y <= 1.;

    if grounded_player && player.velocity.y < 0. {
        player.velocity.y = 0.;
    }

    if keyboard_input.pressed(KeyCode::Space) && grounded_player {
        player.velocity.y += (PLAYRE_JUMP_HEIGHT * -3.0 * PLAYER_GRAVITY).sqrt();
    }

    player.velocity.y += PLAYER_GRAVITY * time.delta_seconds();
    player_transform.translation.y += player.velocity.y * time.delta_seconds();

    if player_transform.translation.y <= 1. {
        player_transform.translation.y = 1.;
    }
}

fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction: Vec3 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction.z -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction.z += 1.;
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
