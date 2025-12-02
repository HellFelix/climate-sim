pub fn my_f(x: f32, y: f32) -> f32 {
    return (1.0 - x * x - y * y).sqrt();
}

pub fn transmition_f(my: f32) -> f32 {
    let omega: f32 = 0.98;
    let tao: f32 = 0.3;
    let t: f32 = (omega * tao) / (2.0 * my);
    return t + (-tao / my).exp();
}

pub fn flux(my: f32) -> f32 {
    let solar_constant: f32 = 1.3608;
    let ro: f32 = 0.05;
    let r: f32 = 0.08;
    return ((solar_constant * my * transmition_f(-my)) / (1.0 - ro * r));
}

pub fn flux_pp(I: Vec<f32>) {}
