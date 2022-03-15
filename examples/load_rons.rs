//! Custom RON asset loading example
//!
//! Spawn some cubes and a camera based on parameters defined in RON files.
//!
//! In real games, you should probably use Bevy's scenes instead, for spawning
//! entities into your world. This example is just for illustrative purposes,
//! not necessarily best practice for this particular use-case.

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_ron::*;

#[derive(serde::Deserialize, TypeUuid)]
#[uuid = "b7f64775-6e72-4080-9ced-167607f1f0b2"]
struct CameraSettingsAsset {
    translation: [f32; 3],
    fov_degrees: f32,
}

#[derive(serde::Deserialize, serde::Serialize, TypeUuid)]
#[uuid = "39ae6c9e-9320-4575-ad28-4cc3f10dd0e8"]
struct CubesAsset {
    color: [f32; 4],
    size: f32,
    positions: Vec<[f32; 3]>,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // load camera settings from "*.camera" files
        .add_plugin(RonAssetPlugin::<CameraSettingsAsset>::new(&["camera"]))
        // load cubes to spawn from "*.cubes" files
        .add_plugin(RonAssetPlugin::<CubesAsset>::new(&["cubes"]))
        .add_startup_system(setup)
        .add_system(init_camera)
        .add_system(spawn_cubes)
        .run();
}

/// Component for our camera entities
#[derive(Component)]
struct CameraSettings {
    handle: Handle<CameraSettingsAsset>,
}

/// Component for our object setting entities
#[derive(Component)]
struct ObjectSettings {
    handle: HandleUntyped,
}

/// Setup System
fn setup(mut commands: Commands, server: Res<AssetServer>) {
    // load the camera settings
    let ass_cam = server.load("settings.camera");

    // spawn placeholder entity for camera
    // it will be initialized when the settings finish loading
    commands.spawn_bundle((CameraSettings { handle: ass_cam },));

    // load configs for objects
    let ass_objs = server.load_folder("objects").unwrap();
    for ass_obj in ass_objs {
        commands.spawn_bundle((ObjectSettings { handle: ass_obj },));
    }
}

/// Initialize camera when the settings are loaded
fn init_camera(
    mut commands: Commands,
    assets: Res<Assets<CameraSettingsAsset>>,
    q: Query<(Entity, &CameraSettings)>,
) {
    for (e, cs) in q.iter() {
        if let Some(s) = assets.get(&cs.handle) {
            // despawn placeholder
            commands.entity(e).despawn();

            // spawn actual camera
            let mut camera = PerspectiveCameraBundle::default();
            camera.transform.translation = s.translation.into();
            camera.transform.look_at(Vec3::ZERO, Vec3::Y);
            camera.perspective_projection.fov = s.fov_degrees * std::f32::consts::PI / 180.0;

            commands.spawn_bundle(camera);
        }
    }
}

/// Spawn cubes when cube description assets are loaded
fn spawn_cubes(
    mut commands: Commands,
    assets: Res<Assets<CubesAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &ObjectSettings)>,
) {
    for (e, os) in q.iter() {
        if let Some(s) = assets.get(&os.handle) {
            // despawn placeholder
            commands.entity(e).despawn();

            // spawn cubes
            for &cube in &s.positions {
                commands.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: s.size })),
                    material: materials.add(Color::from(s.color).into()),
                    transform: Transform::from_translation(cube.into()),
                    ..Default::default()
                });
            }
        }
    }
}
