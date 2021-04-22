#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::multiple_crate_versions,
    clippy::default_trait_access,
    clippy::needless_pass_by_value
)]

use bevy::{audio::AudioPlugin, input::system::exit_on_esc_system, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            ..Default::default()
        })
        //.insert_resource(Msaa { samples: 4 })
        // Adds frame time diagnostics
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // Any plugin can register diagnostics
        //.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        // Uncomment this to add an asset count diagnostics:
        /*.add_plugin(bevy::asset::diagnostic::AssetCountDiagnosticsPlugin::<
            Texture,
        >::default())*/
        // Uncomment this to add some render resource diagnostics:
        //.add_plugin(bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin::default())
        // Uncomment this to add an entity count diagnostics:
        .add_plugin(FlyCameraPlugin)
        .add_plugins_with(DefaultPlugins, |group| group.disable::<AudioPlugin>())
        .add_startup_system(load_models.system())
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(mouse_lock.system())
        .add_system(print_positions.system())
        .run();
}

struct Model;

/// Load models into asset server.
fn load_models(mut commands: Commands, asset_server: Res<AssetServer>) {
    let models: Vec<Handle<Scene>> = vec![
        asset_server.load("models/water.glb#Scene0"),
        asset_server.load("models/water_rocks.glb#Scene0"),
        asset_server.load("models/water_island.glb#Scene0"),
        asset_server.load("models/river_straight.glb#Scene0"),
        asset_server.load("models/river_start.glb#Scene0"),
        asset_server.load("models/river_intersectionH.glb#Scene0"),
        asset_server.load("models/river_intersectionG.glb#Scene0"),
        asset_server.load("models/river_intersectionF.glb#Scene0"),
        asset_server.load("models/river_intersectionE.glb#Scene0"),
        asset_server.load("models/river_intersectionD.glb#Scene0"),
        asset_server.load("models/river_intersectionC.glb#Scene0"),
        asset_server.load("models/river_intersectionB.glb#Scene0"),
        asset_server.load("models/river_intersectionA.glb#Scene0"),
        asset_server.load("models/river_end.glb#Scene0"),
        asset_server.load("models/river_crossing.glb#Scene0"),
        asset_server.load("models/river_cornerSharp.glb#Scene0"),
        asset_server.load("models/river_corner.glb#Scene0"),
        asset_server.load("models/building_water.glb#Scene0"),
        asset_server.load("models/building_wall.glb#Scene0"),
        asset_server.load("models/building_village.glb#Scene0"),
        asset_server.load("models/building_tower.glb#Scene0"),
        asset_server.load("models/building_smelter.glb#Scene0"),
        asset_server.load("models/building_sheep.glb#Scene0"),
        asset_server.load("models/building_mine.glb#Scene0"),
        asset_server.load("models/building_mill.glb#Scene0"),
        asset_server.load("models/building_market.glb#Scene0"),
        asset_server.load("models/building_house.glb#Scene0"),
        asset_server.load("models/building_farm.glb#Scene0"),
        asset_server.load("models/building_dock.glb#Scene0"),
        asset_server.load("models/building_castle.glb#Scene0"),
        asset_server.load("models/building_cabin.glb#Scene0"),
        asset_server.load("models/sand.glb#Scene0"),
        asset_server.load("models/sand_rocks.glb#Scene0"),
        asset_server.load("models/grass.glb#Scene0"),
        asset_server.load("models/grass_hill.glb#Scene0"),
        asset_server.load("models/grass_forest.glb#Scene0"),
        asset_server.load("models/dirt.glb#Scene0"),
        asset_server.load("models/dirt_lumber.glb#Scene0"),
        asset_server.load("models/stone.glb#Scene0"),
        asset_server.load("models/stone_rocks.glb#Scene0"),
        asset_server.load("models/stone_mountain.glb#Scene0"),
        asset_server.load("models/stone_hill.glb#Scene0"),
    ];

    // Roads and units, don't contain the hexagon tile themselves.
    /*
    models.push(asset_server.load("models/unit_wallTower.glb#Scene0"));
    models.push(asset_server.load("models/unit_tree.glb#Scene0"));
    models.push(asset_server.load("models/unit_tower.glb#Scene0"));
    models.push(asset_server.load("models/unit_mill.glb#Scene0"));
    models.push(asset_server.load("models/unit_houseLarge.glb#Scene0"));
    models.push(asset_server.load("models/unit_house.glb#Scene0"));
    models.push(asset_server.load("models/unit_boat.glb#Scene0"));
    models.push(asset_server.load("models/path_straight.glb#Scene0"));
    models.push(asset_server.load("models/path_start.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionH.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionG.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionF.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionE.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionD.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionC.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionB.glb#Scene0"));
    models.push(asset_server.load("models/path_intersectionA.glb#Scene0"));
    models.push(asset_server.load("models/path_end.glb#Scene0"));
    models.push(asset_server.load("models/path_crossing.glb#Scene0"));
    models.push(asset_server.load("models/path_cornerSharp.glb#Scene0"));
    models.push(asset_server.load("models/path_corner.glb#Scene0"));
    */

    let mut offset = 0.0;
    for model in models {
        commands
            .spawn_bundle((
                Transform::from_xyz(offset, 0.0, -1.0),
                GlobalTransform::identity(),
                Model {},
            ))
            .with_children(|parent| {
                parent.spawn_scene(model);
            });
        offset += 1.0;
    }
}

/// set up a simple 3D scene
fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera {
            accel: 1.5,
            max_speed: 0.5,
            sensitivity: 3.0,
            friction: 1.0,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
            key_forward: KeyCode::Comma,
            key_backward: KeyCode::O,
            key_left: KeyCode::A,
            key_right: KeyCode::E,
            key_up: KeyCode::Space,
            key_down: KeyCode::LShift,
            enabled: true,
        })
        .insert(Light::default());
}

fn print_positions(keyboard_input: Res<Input<KeyCode>>, query: Query<&Transform, With<Model>>) {
    if keyboard_input.just_pressed(KeyCode::Key0) {
        for transform in query.iter() {
            println!("{:#?}", transform);
        }
    }
}

fn mouse_lock(mut cursor_entered_events: EventReader<CursorEntered>, mut windows: ResMut<Windows>) {
    let window = windows
        .get_primary_mut()
        .expect("Couldn't find primary window.");
    for event in cursor_entered_events.iter() {
        if window.id() == event.id {
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        }
    }
}
