#[inline]
pub fn db_to_gain(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

#[inline]
pub fn gain_to_db(gain: f32) -> f32 {
    if gain > 0.0 {
        20.0 * gain.log10()
    } else {
        -f32::INFINITY
    }
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
