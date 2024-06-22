use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bgel::{GLTFExtender, SpawnAsset};
fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins((DefaultPlugins, GLTFExtender))
        .run();
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(5.0, 5.0, 1.0)
                .looking_at(Vec3::new(0.3, 0.0, 0.0), Vec3::Y),
            ..default()
        },
        BloomSettings::NATURAL,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },

        ..default()
    });

    let my_gltf = assets.load("material_test.glb");

    commands.spawn(SpawnAsset { handle: my_gltf });
    commands.spawn(SceneBundle {
        scene: assets.load("material_test.glb#Scene0"),
        transform: Transform::from_scale(Vec3::splat(0.1)),
        ..Default::default()
    });
}
