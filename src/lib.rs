use bevy::{gltf::Gltf, prelude::*};

pub struct GLTFExtender;
pub struct ExtendedGLTFSpawner;

impl Plugin for GLTFExtender {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_gltf_objects, extend_material));
    }
}

#[derive(Component)]
pub struct SpawnAsset {
    pub handle: Handle<Gltf>,
}

#[derive(Component)]
pub struct AssetExtension {
    asset_index: AssetId<StandardMaterial>,
    emissive_color: Color,
}

pub fn spawn_gltf_objects(
    mut commands: Commands,
    spawn_assets: Query<(Entity, &mut SpawnAsset)>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    for (entity, spawn_asset) in &spawn_assets {
        if let Some(gltf) = assets_gltf.get(&spawn_asset.handle) {
            let source = gltf.source.as_ref().unwrap();
            for material in &gltf.named_materials {
                for raw_material in source.materials() {
                    if let Some(name) = raw_material.name() {
                        if material.0 == name {
                            if let Some(emissive_strength) = raw_material.emissive_strength() {
                                let mut emissive_color: [f32; 3] = raw_material.emissive_factor();
                                emissive_color[0] *= emissive_strength;
                                emissive_color[1] *= emissive_strength;
                                emissive_color[2] *= emissive_strength;
                                commands.spawn(AssetExtension {
                                    asset_index: material.1.id(),
                                    emissive_color: Color::rgb_from_array(emissive_color),
                                });
                            }
                        }
                    }
                }
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn extend_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    extended_assets: Query<(Entity, &AssetExtension)>,
) {
    for (entity, asset_extension) in &extended_assets {
        if let Some(material) = materials.get_mut(asset_extension.asset_index) {
            material.emissive = asset_extension.emissive_color;
        }
        commands.entity(entity).despawn_recursive();
    }
}
