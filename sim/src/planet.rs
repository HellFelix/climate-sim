use bevy::prelude::*;

use crate::consts::*;

#[derive(Resource)]
pub struct PlanetRenderTexture(pub Handle<Image>);

#[derive(Component)]
pub struct Planet {
    pub time: f32,
    pub vx: f32,
    pub vy: f32,
}

impl Default for Planet {
    fn default() -> Self {
        Planet {
            time: 0.,
            vx: 0.,
            vy: 2.,
        }
    }
}

pub fn rotate(mut query: Query<&mut Transform, With<Planet>>, time: Res<Time>) {
    for mut transform in &mut query {
        let forward = transform.forward();
        transform.rotate_axis(forward, time.delta_secs() / 2.);
    }
}

pub fn move_planet(mut planet_query: Query<(&mut Transform, &mut Planet)>) {
    let (mut transform, mut planet) = planet_query.single_mut().unwrap();
    planet.time += PLANET_DT;

    let ecc_anom = mikkola_approximation(planet.time);
    transform.translation.x = A * (ecc_anom.cos() - E);
    transform.translation.y = A * (1. - E.powi(2)).sqrt() * ecc_anom.sin();
}

fn mikkola_approximation(t: f32) -> f32 {
    let m = N * (t - PER_TIME);

    let alpha = 3. * (1. - E) / (1. + E);
    let beta = m / (1. + E);

    let b = (beta + (beta.powi(2) + alpha.powi(3)).sqrt()).powf(1. / 3.);

    // Solve quadratic
    let z = b - alpha / b;

    // Approximate eccentric anomaly
    let mut res = m + E * (3. * z - 4. * z.powi(3));

    // One Newton refinement
    res -= (res - E * res.sin() - m) / (1. - E * res.sin());
    res
}

#[derive(Component)]
pub struct PlanetStats;

pub fn update_stats(
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
