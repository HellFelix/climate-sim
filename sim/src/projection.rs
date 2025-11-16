use bevy::{
    ecs::resource::Resource,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::Material2d,
};

#[derive(Resource)]
pub struct PlanetEntity(Entity);

#[derive(Resource)]
pub struct SunEntity(Entity);

#[derive(Asset, AsBindGroup, TypePath, Clone)]
pub struct MinimapMaterial {
    // equirectangular base color (planet map)
    #[texture(0)]
    #[sampler(1)]
    pub(crate) albedo: Handle<Image>,

    // packed uniforms
    #[uniform(2)]
    pub(crate) params: MinimapParams,
}

#[derive(Component)]
pub struct MiniMapHandle(pub Handle<MinimapMaterial>);

#[derive(Clone, Copy, ShaderType)]
pub struct MinimapParams {
    pub(crate) world_to_planet: Mat3, // 3x3
    pub(crate) light_dir_world: Vec3, // normalized
    pub(crate) ambient: f32,          // 0..1
    pub(crate) use_mercator: u32,     // 0 or 1
    pub(crate) _pad: Vec3,            // padding (unused)
}

impl Material2d for MinimapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/projection.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/projection.wgsl".into()
    }
}
