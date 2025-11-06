use std::f32::consts::PI;

use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::render::render_resource::{TextureDescriptor, TextureUsages};
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use anyhow;

const G: f32 = 1.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WireframePlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (rotate, move_planet_kepler, update_stats, toggle_wireframe),
        )
        .run();
}

#[derive(Component)]
struct Star;

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Planet {
    pub mass: f32,
    pub vx: f32,
    pub vy: f32,
}
impl Planet {
    pub fn mu(&self) -> f32 {
        G * self.mass
    }
}

impl Default for Planet {
    fn default() -> Self {
        Planet {
            mass: 8.,
            vx: 0.,
            vy: 2.,
        }
    }
}

#[derive(Component)]
struct PlanetStats;

#[derive(Resource)]
struct MiniCamTexture(Handle<Image>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 3D setup
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let planet_mesh = meshes.add(Sphere::default().mesh().uv(32, 18));
    let star_mesh = meshes.add(Sphere::default().mesh().uv(40, 20));

    commands.spawn((
        Mesh3d(planet_mesh),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(3., 0., 0.),
        Planet::default(),
    ));
    commands.spawn((
        Mesh3d(star_mesh),
        MeshMaterial3d(debug_material.clone()),
        Transform::from_xyz(0., 0., 0.),
        Star,
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 0., 30.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
        PlanetStats,
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Planet>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}

// fn move_planet_circular(mut query: Query<(&mut Transform, &Planet)>, time: Res<Time>) {
//     for (mut transform, planet) in &mut query {
//         let x = transform.translation.x;
//         let y = transform.translation.y;
//
//         let dx = -planet.omega * y * time.delta_secs();
//         let dy = planet.omega * x * time.delta_secs();
//
//         transform.translation.x += dx;
//         transform.translation.y += dy;
//         // let theta = y.atan2(x);
//         //
//         // transform.translation.x = planet.r * (theta + planet.omega * time.delta_secs()).cos();
//         // transform.translation.y = planet.r * (theta + planet.omega * time.delta_secs()).sin();
//         //move_polar(&mut transform, PI / 8., 1.);
//     }
// }

fn update_stats(
    planet_query: Query<(&Transform, &Planet)>,
    mut text_query: Query<&mut Text, With<PlanetStats>>,
) {
    let (transform, planet) = planet_query.single().unwrap();
    let mut text = text_query.single_mut().unwrap();

    let r = (transform.translation.x.powi(2) + transform.translation.y.powi(2)).sqrt();
    let angle = transform.translation.y.atan2(transform.translation.x);
    let speed = (planet.vx.powi(2) + planet.vy.powi(2)).sqrt();

    text.0 = format!("Planet Stats:\nDistance: {r}\nAngle: {angle}\nSpeed: {speed}");
}

fn move_planet_kepler(mut query: Query<(&mut Transform, &mut Planet)>, time: Res<Time>) {
    let (mut transform, mut planet) = query.single_mut().unwrap();
    let x = transform.translation.x;
    let y = transform.translation.y;
    let dvx = -planet.mu() * x / (x.powi(2) + y.powi(2)).powf(3. / 2.) * time.delta_secs();
    let dvy = -planet.mu() * y / (x.powi(2) + y.powi(2)).powf(3. / 2.) * time.delta_secs();

    planet.vx += dvx;
    planet.vy += dvy;

    transform.translation.x += planet.vx * time.delta_secs();
    transform.translation.y += planet.vy * time.delta_secs();
}

fn move_polar(transform: &mut Transform, theta: f32, r: f32) {
    move_cartesian(transform, r * theta.cos(), r * theta.sin());
}

fn move_cartesian(transform: &mut Transform, x: f32, y: f32) {
    transform.translation.x += x;
    transform.translation.y += y;
}

// Debug colors
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 103, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
