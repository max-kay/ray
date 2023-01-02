use std::f32::consts::PI;

use parry3d::math::Vector;

pub fn rand_unit_vec_pos_z() -> Vector<f32> {
    let theta = 2.0 * PI * rand::random::<f32>();
    let phi = (rand::random::<f32>()).acos();
    Vector::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos())
}
