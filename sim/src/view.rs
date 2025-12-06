use std::f32::consts::PI;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};
use bevy_capture::{CameraTargetHeadless, CaptureBundle};
use ndarray::arr2;

use crate::{
    consts::{HEIGHT, ROTATION_SPEED, TRANSLATION_SPEED, WIDTH},
    planet::{Planet, PlanetRenderTexture},
    temp::TempMap,
};

#[derive(Component, Clone, Copy)]
pub enum ViewPoint {
    SolarSystem,
    Planet(Vec3),
    FreeCam,
}

pub fn solar_system_transform() -> Transform {
    Transform::from_xyz(0., 0., 30.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y)
}

#[derive(Component)]
pub struct MainCam;

#[derive(Component)]
pub struct MapCam;

pub fn toggle_view(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ViewPoint>,
    mut q3d: Query<&mut Camera, With<MainCam>>,
    mut q2d: Query<&mut Camera, (With<MapCam>, Without<MainCam>)>,
) {
    let mut view_point = query.single_mut().unwrap();
    if keyboard.just_pressed(KeyCode::KeyO) {
        *view_point = ViewPoint::SolarSystem;
    } else if keyboard.just_pressed(KeyCode::KeyP) {
        *view_point = ViewPoint::Planet(5. * Vec3::X);
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

pub fn physics_control(keyboard: Res<ButtonInput<KeyCode>>, mut time: ResMut<Time<Virtual>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if time.is_paused() {
            time.unpause();
        } else {
            time.pause();
        }
    }
}

pub fn update_camera(
    mut camera_query: Query<(&mut Transform, &mut ViewPoint), Without<Planet>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    planet_query: Query<&Transform, With<Planet>>,
) {
    let (mut camera_transform, view_point) = camera_query.single_mut().unwrap();
    let planet_transform = planet_query.single().unwrap();

    match view_point.into_inner() {
        ViewPoint::SolarSystem => {
            *camera_transform = solar_system_transform();
        }
        &mut ViewPoint::Planet(ref mut offset) => {
            set_view_planet(&mut camera_transform, planet_transform, keyboard, offset)
        }
        ViewPoint::FreeCam => set_view_free_cam(&mut camera_transform, keyboard),
    }
}

fn set_view_planet(
    camera_transform: &mut Transform,
    planet_transform: &Transform,
    keyboard: Res<ButtonInput<KeyCode>>,
    offset: &mut Vec3,
) {
    camera_transform.translation = planet_transform.translation - *offset;

    let translation = if keyboard.pressed(KeyCode::ShiftLeft) {
        -offset.normalize() * TRANSLATION_SPEED
    } else if keyboard.pressed(KeyCode::Space) {
        offset.normalize() * TRANSLATION_SPEED
    } else {
        Vec3::ZERO
    };

    *offset += translation;

    let rotation = if keyboard.pressed(KeyCode::ArrowRight) {
        Quat::from_axis_angle(camera_transform.up().as_vec3(), ROTATION_SPEED)
    } else if keyboard.pressed(KeyCode::ArrowLeft) {
        Quat::from_axis_angle(camera_transform.down().as_vec3(), ROTATION_SPEED)
    } else if keyboard.pressed(KeyCode::ArrowUp) {
        Quat::from_axis_angle(camera_transform.left().as_vec3(), ROTATION_SPEED)
    } else if keyboard.pressed(KeyCode::ArrowDown) {
        Quat::from_axis_angle(camera_transform.right().as_vec3(), ROTATION_SPEED)
    } else {
        Quat::IDENTITY
    };

    *offset = rotation * *offset;

    camera_transform.look_at(planet_transform.translation, Vec3::Z);
}

fn set_view_free_cam(camera_transform: &mut Transform, keyboard: Res<ButtonInput<KeyCode>>) {
    // Translations
    if keyboard.pressed(KeyCode::KeyW) {
        let step = camera_transform.forward().as_vec3().normalize() * TRANSLATION_SPEED;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::KeyS) {
        let step = camera_transform.back().as_vec3().normalize() * TRANSLATION_SPEED;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::KeyD) {
        let step = camera_transform.right().as_vec3().normalize() * TRANSLATION_SPEED;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::KeyA) {
        let step = camera_transform.left().as_vec3().normalize() * TRANSLATION_SPEED;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::Space) {
        let step = camera_transform.up().as_vec3().normalize() * TRANSLATION_SPEED;
        camera_transform.translation += step;
    } else if keyboard.pressed(KeyCode::ShiftLeft) {
        let step = camera_transform.down().as_vec3().normalize() * TRANSLATION_SPEED;
        camera_transform.translation += step;
    }
    // Rotations
    else if keyboard.pressed(KeyCode::ArrowUp) {
        camera_transform.rotate_axis(camera_transform.right(), ROTATION_SPEED);
    } else if keyboard.pressed(KeyCode::ArrowDown) {
        camera_transform.rotate_axis(camera_transform.left(), ROTATION_SPEED);
    } else if keyboard.pressed(KeyCode::ArrowRight) {
        camera_transform.rotate_axis(camera_transform.down(), ROTATION_SPEED);
    } else if keyboard.pressed(KeyCode::ArrowLeft) {
        camera_transform.rotate_axis(camera_transform.up(), ROTATION_SPEED);
    }
}

#[derive(Component)]
pub struct SimulationSpecs {
    pub record: bool,
}

#[derive(Component)]
pub struct OverlayCamera2d;

#[derive(Component)]
struct OverlayRoot;

// Spawn cameras
pub fn setup_cameras(
    mut commands: Commands,
    planet_rt: Res<PlanetRenderTexture>,
    mut images: ResMut<Assets<Image>>,
    sim_specs_query: Query<&SimulationSpecs>,
) {
    let sim_specs = sim_specs_query.single().unwrap();

    // 3D camera
    if sim_specs.record {
        commands.spawn((
            Camera3d::default(),
            MainCam,
            Camera {
                is_active: true,
                ..default()
            }
            .target_headless(3840, 2160, &mut images),
            CaptureBundle::default(),
            ViewPoint::Planet(5. * Vec3::X),
            solar_system_transform(),
        ));
    } else {
        commands.spawn((
            Camera3d::default(),
            MainCam,
            Camera {
                is_active: true,
                ..default()
            },
            ViewPoint::SolarSystem, // Set to solar system by default
            solar_system_transform(),
        ));
    }

    // 2D overlay camera
    let cam_2d = commands
        .spawn((
            Camera2d,
            MapCam,
            if sim_specs.record {
                Camera {
                    order: 10,
                    is_active: false,
                    ..default()
                }
                .target_headless(3840, 2160, &mut images)
            } else {
                Camera {
                    order: 10,
                    is_active: false,
                    ..default()
                }
            },
            OverlayCamera2d,
        ))
        .id();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            UiTargetCamera(cam_2d),
            OverlayRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(planet_rt.0.clone()),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
            ));
        });
}

pub fn setup_texture(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut temp_map = TempMap::new(arr2(&[[0.; HEIGHT]; WIDTH]));

    // temp_map.set_heat(|phi, _theta| if phi < PI { 200. } else { 0. });
    temp_map.set_heat(|_theta, _phi| 0.);
    let mut img = Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &temp_map.get_heat_texture(),
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );

    img.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    commands.spawn(temp_map);
    let rt_handle = images.add(img);
    commands.insert_resource(PlanetRenderTexture(rt_handle));
}
