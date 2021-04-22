#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::multiple_crate_versions,
    clippy::default_trait_access,
    clippy::needless_pass_by_value
)]

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(FlyCameraPlugin)
        .add_plugins_with(DefaultPlugins, |group| {
            group
                .disable::<bevy::audio::AudioPlugin>()
                .disable::<bevy::diagnostic::DiagnosticsPlugin>()
                .disable::<bevy::sprite::SpritePlugin>()
                .disable::<bevy::text::TextPlugin>()
                .disable::<bevy::ui::UiPlugin>()
                .disable::<bevy::gilrs::GilrsPlugin>()
        })
        .add_startup_system(load_tiles.system())
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(mouse_lock.system())
        .add_system(print_positions.system())
        .run();
}

struct Tile;

/// Load tiles into asset server.
fn load_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tiles: Vec<Handle<Scene>> = vec![
        asset_server.load("tiles/water.glb#Scene0"),
        asset_server.load("tiles/water_rocks.glb#Scene0"),
        asset_server.load("tiles/water_island.glb#Scene0"),
        asset_server.load("tiles/river_straight.glb#Scene0"),
        asset_server.load("tiles/river_start.glb#Scene0"),
        asset_server.load("tiles/river_intersectionH.glb#Scene0"),
        asset_server.load("tiles/river_intersectionG.glb#Scene0"),
        asset_server.load("tiles/river_intersectionF.glb#Scene0"),
        asset_server.load("tiles/river_intersectionE.glb#Scene0"),
        asset_server.load("tiles/river_intersectionD.glb#Scene0"),
        asset_server.load("tiles/river_intersectionC.glb#Scene0"),
        asset_server.load("tiles/river_intersectionB.glb#Scene0"),
        asset_server.load("tiles/river_intersectionA.glb#Scene0"),
        asset_server.load("tiles/river_end.glb#Scene0"),
        asset_server.load("tiles/river_crossing.glb#Scene0"),
        asset_server.load("tiles/river_cornerSharp.glb#Scene0"),
        asset_server.load("tiles/river_corner.glb#Scene0"),
        asset_server.load("tiles/building_water.glb#Scene0"),
        asset_server.load("tiles/building_wall.glb#Scene0"),
        asset_server.load("tiles/building_village.glb#Scene0"),
        asset_server.load("tiles/building_tower.glb#Scene0"),
        asset_server.load("tiles/building_smelter.glb#Scene0"),
        asset_server.load("tiles/building_sheep.glb#Scene0"),
        asset_server.load("tiles/building_mine.glb#Scene0"),
        asset_server.load("tiles/building_mill.glb#Scene0"),
        asset_server.load("tiles/building_market.glb#Scene0"),
        asset_server.load("tiles/building_house.glb#Scene0"),
        asset_server.load("tiles/building_farm.glb#Scene0"),
        asset_server.load("tiles/building_dock.glb#Scene0"),
        asset_server.load("tiles/building_castle.glb#Scene0"),
        asset_server.load("tiles/building_cabin.glb#Scene0"),
        asset_server.load("tiles/sand.glb#Scene0"),
        asset_server.load("tiles/sand_rocks.glb#Scene0"),
        asset_server.load("tiles/grass.glb#Scene0"),
        asset_server.load("tiles/grass_hill.glb#Scene0"),
        asset_server.load("tiles/grass_forest.glb#Scene0"),
        asset_server.load("tiles/dirt.glb#Scene0"),
        asset_server.load("tiles/dirt_lumber.glb#Scene0"),
        asset_server.load("tiles/stone.glb#Scene0"),
        asset_server.load("tiles/stone_rocks.glb#Scene0"),
        asset_server.load("tiles/stone_mountain.glb#Scene0"),
        asset_server.load("tiles/stone_hill.glb#Scene0"),
    ];

    // Roads and units, don't contain the hexagon tile themselves.
    /*
    tiles.push(asset_server.load("tiles/unit_wallTower.glb#Scene0"));
    tiles.push(asset_server.load("tiles/unit_tree.glb#Scene0"));
    tiles.push(asset_server.load("tiles/unit_tower.glb#Scene0"));
    tiles.push(asset_server.load("tiles/unit_mill.glb#Scene0"));
    tiles.push(asset_server.load("tiles/unit_houseLarge.glb#Scene0"));
    tiles.push(asset_server.load("tiles/unit_house.glb#Scene0"));
    tiles.push(asset_server.load("tiles/unit_boat.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_straight.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_start.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionH.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionG.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionF.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionE.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionD.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionC.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionB.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_intersectionA.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_end.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_crossing.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_cornerSharp.glb#Scene0"));
    tiles.push(asset_server.load("tiles/path_corner.glb#Scene0"));
    */

    let mut offset = 0.0;
    for tile in tiles {
        commands
            .spawn_bundle((
                Transform::from_xyz(offset, 0.0, -1.0),
                GlobalTransform::identity(),
                Tile {},
            ))
            .with_children(|parent| {
                parent.spawn_scene(tile);
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
        .insert(Light {
            color: Color::rgb(1.0, 1.0, 1.0),
            depth: 0.1..50.0,
            fov: f32::to_radians(60.0),
            intensity: 200.0,
            range: 50.0,
        });
}

fn print_positions(keyboard_input: Res<Input<KeyCode>>, query: Query<&Transform, With<Tile>>) {
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
