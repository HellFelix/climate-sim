use bevy::prelude::*;

use crate::planet::Planet;

#[derive(Component, Clone, Copy)]
pub enum ViewPoint {
    SolarSystem,
    Planet,
    FreeCam,
}

pub fn solar_system_transform() -> Transform {
    Transform::from_xyz(0., 0., 30.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y)
}

pub fn toggle_view(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ViewPoint>,
    mut q3d: Query<&mut Camera, With<Camera3d>>,
    mut q2d: Query<&mut Camera, Without<Camera3d>>,
) {
    let mut view_point = query.single_mut().unwrap();
    if keyboard.just_pressed(KeyCode::KeyO) {
        *view_point = ViewPoint::SolarSystem;
    } else if keyboard.just_pressed(KeyCode::KeyP) {
        *view_point = ViewPoint::Planet;
    } else if keyboard.just_pressed(KeyCode::KeyF) {
        *view_point = ViewPoint::FreeCam;
    } else if keyboard.just_pressed(KeyCode::KeyM) {
        // Doesn't change ViewPoint, only toggles map.
        let mut cam3d = q3d.single_mut().unwrap();
        let mut cam2d = q2d.single_mut().unwrap();

        let use_3d = !cam3d.is_active;

        info!("Toggling");
        cam3d.is_active = use_3d;
        cam2d.is_active = !use_3d;
    }
}

pub fn update_camera(
    mut camera_query: Query<(&mut Transform, &ViewPoint), Without<Planet>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    planet_query: Query<&Transform, With<Planet>>,
) {
    let (mut camera_transform, view_point) = camera_query.single_mut().unwrap();
    let planet_transform = planet_query.single().unwrap();

    match view_point {
        ViewPoint::SolarSystem => {
            *camera_transform = solar_system_transform();
        }
        ViewPoint::Planet => set_view_planet(&mut camera_transform, planet_transform),
        ViewPoint::FreeCam => set_view_free_cam(&mut camera_transform, keyboard),
    }
}

fn set_view_planet(camera_transform: &mut Transform, planet_transform: &Transform) {
    camera_transform.translation = planet_transform.translation;
    camera_transform.translation.x -= 5.;

    camera_transform.look_at(planet_transform.translation, Vec3::Z);
}

fn set_view_free_cam(camera_transform: &mut Transform, keyboard: Res<ButtonInput<KeyCode>>) {
    let translation_speed = 0.1;
    let rotation_speed = 0.02;

    // Translations
    if keyboard.pressed(KeyCode::KeyW) {
        let step = camera_transform.forward().as_vec3().normalize() * translation_speed;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::KeyS) {
        let step = camera_transform.back().as_vec3().normalize() * translation_speed;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::KeyD) {
        let step = camera_transform.right().as_vec3().normalize() * translation_speed;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::KeyA) {
        let step = camera_transform.left().as_vec3().normalize() * translation_speed;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::Space) {
        let step = camera_transform.up().as_vec3().normalize() * translation_speed;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::ShiftLeft) {
        let step = camera_transform.down().as_vec3().normalize() * translation_speed;
        camera_transform.translation += step;
    }
    // Rotations
    else if keyboard.pressed(KeyCode::ArrowUp) {
        camera_transform.rotate_axis(camera_transform.right(), rotation_speed);
    } else if keyboard.pressed(KeyCode::ArrowDown) {
        camera_transform.rotate_axis(camera_transform.left(), rotation_speed);
    } else if keyboard.pressed(KeyCode::ArrowRight) {
        camera_transform.rotate_axis(camera_transform.down(), rotation_speed);
    } else if keyboard.pressed(KeyCode::ArrowLeft) {
        camera_transform.rotate_axis(camera_transform.up(), rotation_speed);
    }
}

#[derive(Component)]
pub struct OverlayCamera2d;

#[derive(Component)]
struct OverlayRoot;

// Spawn cameras
pub fn setup_cameras(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 3D camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            is_active: true,
            ..default()
        },
        ViewPoint::SolarSystem, // Set to solar system by default
        solar_system_transform(),
    ));

    // 2D overlay camera
    let cam_2d = commands
        .spawn((
            Camera2d,
            Camera {
                order: 10,
                is_active: false,
                ..default()
            },
            OverlayCamera2d,
        ))
        .id();

    // Fullscreen image *targeted at that 2D camera*
    let texture: Handle<Image> = asset_server.load("world.jpg");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            UiTargetCamera(cam_2d), // <- key line!
            OverlayRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(texture),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            ));
        });
}
