use bevy::prelude::*;

use crate::consts::G;

#[derive(Component)]
pub struct Planet {
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

pub fn rotate(mut query: Query<&mut Transform, With<Planet>>, time: Res<Time>) {
    for mut transform in &mut query {
        let forward = transform.forward();
        transform.rotate_axis(forward, time.delta_secs() / 2.);
    }
}

pub fn move_planet_kepler(mut query: Query<(&mut Transform, &mut Planet)>, time: Res<Time>) {
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
