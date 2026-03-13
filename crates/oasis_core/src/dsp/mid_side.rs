#[inline]
pub fn encode(left: f32, right: f32) -> (f32, f32) {
    let mid = (left + right) * 0.5;
    let side = (left - right) * 0.5;
    (mid, side)
}

#[inline]
pub fn decode(mid: f32, side: f32) -> (f32, f32) {
    let left = mid + side;
    let right = mid - side;
    (left, right)
}

pub fn encode_block(left: &[f32], right: &[f32], mid: &mut [f32], side: &mut [f32]) {
    for i in 0..left.len() {
        mid[i] = (left[i] + right[i]) * 0.5;
        side[i] = (left[i] - right[i]) * 0.5;
    }
}

pub fn decode_block(mid: &[f32], side: &[f32], left: &mut [f32], right: &mut [f32]) {
    for i in 0..mid.len() {
        left[i] = mid[i] + side[i];
        right[i] = mid[i] - side[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let l = 0.7_f32;
        let r = 0.3_f32;
        let (m, s) = encode(l, r);
        let (l2, r2) = decode(m, s);
        assert!((l - l2).abs() < 1e-6);
        assert!((r - r2).abs() < 1e-6);
    }

    #[test]
    fn test_mono_signal_has_no_side() {
        let (_, s) = encode(0.5, 0.5);
        assert!((s).abs() < 1e-6);
    }
}
