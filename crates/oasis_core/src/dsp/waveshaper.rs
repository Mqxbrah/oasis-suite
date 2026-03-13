use crate::util::denormal::sanitize;

#[inline]
pub fn soft_clip(x: f32, drive: f32) -> f32 {
    let driven = x * (1.0 + drive * 9.0);
    sanitize((2.0 / std::f32::consts::PI) * driven.atan())
}

#[inline]
pub fn tape_saturate(x: f32, drive: f32) -> f32 {
    let driven = x * (1.0 + drive * 5.0);
    sanitize(driven.tanh())
}

#[inline]
pub fn tube_saturate(x: f32, drive: f32) -> f32 {
    let amount = drive * 4.0;
    let driven = x * (1.0 + amount);
    let positive = if driven >= 0.0 {
        1.0 - (-driven).exp()
    } else {
        -(1.0 - driven.exp()) * 0.7
    };
    sanitize(positive)
}

#[inline]
pub fn transistor_clip(x: f32, drive: f32) -> f32 {
    let driven = x * (1.0 + drive * 8.0);
    sanitize(driven.clamp(-1.0, 1.0))
}

#[inline]
pub fn digital_fold(x: f32, drive: f32) -> f32 {
    let driven = x * (1.0 + drive * 6.0);
    let folded = if driven.abs() > 1.0 {
        let wrapped = (driven.abs() - 1.0) % 4.0;
        let v = if wrapped < 2.0 { 1.0 - wrapped } else { wrapped - 3.0 };
        sanitize(v * driven.signum())
    } else {
        driven
    };
    folded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soft_clip_bounded() {
        for drive in [0.0, 0.5, 1.0] {
            for input in [-2.0, -1.0, 0.0, 0.5, 1.0, 2.0] {
                let out = soft_clip(input, drive);
                assert!(out.abs() <= 1.01, "soft_clip should stay bounded");
            }
        }
    }

    #[test]
    fn test_tape_saturate_passes_silence() {
        assert!((tape_saturate(0.0, 0.5)).abs() < 1e-6);
    }
}
