use std::f32::consts::PI;

use bevy::prelude::*;

mod consts;
mod planet;
use crate::planet::{Planet, PlanetRenderTexture, PlanetStats};
mod rk4;
mod temp;
mod view;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),))
        .add_systems(
            Startup,
            (view::setup_texture, setup_system, view::setup_cameras).chain(),
        )
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
        .add_systems(FixedUpdate, temp::apply_heat_eq)
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .run();
}

#[derive(Component)]
struct Star;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    render_tex: Res<PlanetRenderTexture>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(render_tex.0.clone()),
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
