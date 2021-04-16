#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, clippy::default_trait_access)]

use bevy::audio::AudioPlugin;
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;

use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(FlyCameraPlugin)
        .add_plugins_with(DefaultPlugins, |group| group.disable::<AudioPlugin>())
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_system(mouse_lock_system.system())
        .run();
}

/// set up a simple 3D scene
#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_scene(asset_server.load("models/building_cabin.glb#Scene0"));

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

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
        });
}

fn mouse_lock_system(
    mut cursor_entered_events: EventReader<CursorEntered>,
    mut windows: ResMut<Windows>,
) {
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
