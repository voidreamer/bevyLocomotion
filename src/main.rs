use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;

mod locomotion;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(locomotion::LocomotionPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, locomotion::controller::player_movement_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Light
    commands.spawn((
        DirectionalLight {
            color: Color::linear_rgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        CascadeShadowConfigBuilder {
            num_cascades: 1,
            first_cascade_far_bound: 40.0,
            ..default()
        }.build(),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Character - we'll load your GLTF model
    commands.spawn((
        SceneRoot(asset_server.load("models/character.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        locomotion::CharacterController {
            velocity: Vec3::ZERO,
            rotational_velocity: 0.0,
        },
    ));
}
