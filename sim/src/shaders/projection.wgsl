const PI: f32 = 3.141592653589793;

// Uniforms you pass from Bevy
struct Params {
    planet_to_world: mat3x3<f32>; // or a quat you convert to mat3
    light_dir_world: vec3<f32>;   // normalized, direction TOWARD the surface (choose convention)
    use_mercator: u32;            // 0=equirect, 1=mercator
    ambient: f32;                 // e.g. 0.15
}
@group(1) @binding(0) var<uniform> params: Params;

@group(0) @binding(0) var albedo_tex: texture_2d<f32>;
@group(0) @binding(1) var albedo_smp: sampler;

// Convert map uv -> (lambda, phi)
fn uv_to_lambda_phi(uv: vec2<f32>, mercator: bool) -> vec2<f32> {
    let lambda = uv.x * 2.0 * PI - PI;
    if (!mercator) {
        let phi = (0.5 - uv.y) * PI;
        return vec2<f32>(lambda, phi);
    } else {
        // mercator inverse
        let t = exp(PI * (1.0 - 2.0 * uv.y));
        let phi = 2.0 * atan(t) - 0.5 * PI;
        return vec2<f32>(lambda, phi);
    }
}

fn lambda_phi_to_normal(lambda: f32, phi: f32) -> vec3<f32> {
    let c = cos(phi);
    return vec3<f32>(c * cos(lambda), sin(phi), c * sin(lambda));
}

@fragment
fn fs_main(@location(0) map_uv: vec2<f32>) -> @location(0) vec4<f32> {
    // 1) map uv -> sphere normal in planet space
    let lp = uv_to_lambda_phi(map_uv, params.use_mercator != 0u);
    let n_p = lambda_phi_to_normal(lp.x, lp.y);

    // 2) choose albedo uv
    var albedo_uv = map_uv; // equirect output sampling
    if (params.use_mercator != 0u) {
        // map output is mercator, but albedo is equirectangular:
        let v_eq = 0.5 - lp.y / PI;
        albedo_uv = vec2<f32>(map_uv.x, clamp(v_eq, 0.0, 1.0));
    }

    // 3) sample albedo
    let albedo = textureSample(albedo_tex, albedo_smp, albedo_uv).rgb;

    // 4) light dir in planet space
    // If params.planet_to_world is provided, invert it once on CPU and pass world_to_planet instead
    // For brevity assume we passed world_to_planet (w2p) as the matrix:
    let w2p = inverse(params.planet_to_world);
    let l_p = normalize( w2p * params.light_dir_world );

    // 5) lighting
    let nl = max(dot(n_p, l_p), 0.0);
    let ambient = params.ambient;
    let lit = albedo * (ambient + (1.0 - ambient) * nl);

    return vec4<f32>(lit, 1.0);
}


struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@location(0) in_pos: vec3<f32>, @location(1) in_uv: vec2<f32>) -> VsOut {
    var out: VsOut;
    out.pos = vec4<f32>(in_pos.xy, 0.0, 1.0); // drop Z
    out.uv = in_uv;
    return out;
}
