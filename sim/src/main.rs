use std::f32::consts::PI;

use bevy::image::ImageSampler;
use bevy::math::FloatOrd;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::render::camera::{ImageRenderTarget, RenderTarget};
use bevy::render::render_resource::TextureUsages;
use bevy::render::view::RenderLayers;
use bevy::sprite::Material2dPlugin;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::planet::{Planet, PlanetStats};
use crate::projection::{MiniMapHandle, MinimapMaterial};
use crate::view::{ViewPoint, solar_system_transform};

mod consts;
mod planet;
mod projection;
mod view;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WireframePlugin::default(),
            Material2dPlugin::<MinimapMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                planet::rotate,
                planet::move_planet_kepler,
                planet::update_stats,
                view::toggle_view,
                view::update_camera,
            ),
        )
        .run();
}

#[derive(Component)]
struct Star;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("world.jpg");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: false,
        ..default()
    });

    let planet_mesh = meshes.add(Sphere::default().mesh().uv(32, 18));
    let star_mesh = meshes.add(Sphere::default().mesh().uv(40, 20));

    let mut transform = Transform::from_xyz(3., 0., 0.);
    transform.rotate_axis(Dir3::Y, 23. * PI / 180.);
    commands.spawn((
        Mesh3d(planet_mesh),
        MeshMaterial3d(material_handle),
        transform,
        Planet::default(),
    ));
    commands.spawn((
        Mesh3d(star_mesh),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::linear_rgb(1., 0.98, 0.20),
            unlit: true,
            ..default()
        })),
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
        ViewPoint::SolarSystem, // Set to solar system by default
        solar_system_transform(),
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

// // Debug colors
// fn uv_debug_texture() -> Image {
//     const TEXTURE_SIZE: usize = 8;
//
//     let mut palette: [u8; 32] = [
//         255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 103, 255, 102, 255,
//         198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
//     ];
//
//     let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
//     for y in 0..TEXTURE_SIZE {
//         let offset = TEXTURE_SIZE * y * 4;
//         texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
//         palette.rotate_right(4);
//     }
//
//     Image::new_fill(
//         Extent3d {
//             width: TEXTURE_SIZE as u32,
//             height: TEXTURE_SIZE as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         &texture_data,
//         TextureFormat::Rgba8UnormSrgb,
//         RenderAssetUsages::RENDER_WORLD,
//     )
// }
