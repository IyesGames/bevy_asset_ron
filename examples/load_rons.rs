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

#[derive(serde::Deserialize)]
#[derive(TypeUuid)]
#[uuid = "b7f64775-6e72-4080-9ced-167607f1f0b2"]
struct CameraSettingsAsset {
    translation: [f32; 3],
    fov_degrees: f32,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(TypeUuid)]
#[uuid = "39ae6c9e-9320-4575-ad28-4cc3f10dd0e8"]
struct CubesAsset {
    color: [f32; 4],
    size: f32,
    positions: Vec<[f32; 3]>,
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // load camera settings from "*.camera" files
        .add_plugin(RonAssetPlugin::<CameraSettingsAsset>::new(&["camera"]))
        // load cubes to spawn from "*.cubes" files
        .add_plugin(RonAssetPlugin::<CubesAsset>::new(&["cubes"]))
        .add_startup_system(setup.system())
        .add_system(init_camera.system())
        .add_system(spawn_cubes.system())
        .run();
}

/// Component for our camera entities
struct CameraSettings {
    handle: Handle<CameraSettingsAsset>,
}

/// Component for our object setting entities
struct ObjectSettings {
    handle: HandleUntyped,
}

/// Setup System
fn setup(
    commands: &mut Commands,
    server: Res<AssetServer>)
{
    // load the camera settings
    let ass_cam = server.load("settings.camera");

    // spawn placeholder entity for camera
    // it will be initialized when the settings finish loading
    commands.spawn((CameraSettings { handle: ass_cam },));

    // load configs for objects
    let ass_objs = server.load_folder("objects").unwrap();
    for ass_obj in ass_objs {
        commands.spawn((ObjectSettings { handle: ass_obj },));
    }
}

/// Initialize camera when the settings are loaded
fn init_camera(
    commands: &mut Commands,
    assets: Res<Assets<CameraSettingsAsset>>,
    q: Query<(Entity, &CameraSettings)>,
) {
    for (e, cs) in q.iter() {
        if let Some(s) = assets.get(&cs.handle) {
            // despawn placeholder
            commands.despawn(e);

            // spawn actual camera
            let mut camera = Camera3dBundle::default();
            camera.transform.translation = s.translation.into();
            camera.transform.look_at(Vec3::zero(), Vec3::unit_y());
            camera.perspective_projection.fov = s.fov_degrees * std::f32::consts::PI / 180.0;
            
            commands.spawn(camera);
        }
    }
}

/// Spawn cubes when cube description assets are loaded
fn spawn_cubes(
    commands: &mut Commands,
    assets: Res<Assets<CubesAsset>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &ObjectSettings)>,
) {
    for (e, os) in q.iter() {
        if let Some(s) = assets.get(&os.handle) {
            // despawn placeholder
            commands.despawn(e);

            // spawn cubes
            for &cube in &s.positions {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: s.size })),
                    material: materials.add(Color::from(s.color).into()),
                    transform: Transform::from_translation(cube.into()),
                    ..Default::default()
                });
            }
        }
    }
}
