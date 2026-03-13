use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct BiquadCoeffs {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl Default for BiquadCoeffs {
    fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        }
    }
}

pub struct Biquad {
    coeffs: BiquadCoeffs,
    z1: f32,
    z2: f32,
}

impl Biquad {
    pub fn new() -> Self {
        Self {
            coeffs: BiquadCoeffs::default(),
            z1: 0.0,
            z2: 0.0,
        }
    }

    pub fn set_highpass(&mut self, freq_hz: f32, q: f32, sample_rate: f32) {
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let a0 = 1.0 + alpha;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: ((1.0 + cos_omega) / 2.0) * a0_inv,
            b1: (-(1.0 + cos_omega)) * a0_inv,
            b2: ((1.0 + cos_omega) / 2.0) * a0_inv,
            a1: (-2.0 * cos_omega) * a0_inv,
            a2: (1.0 - alpha) * a0_inv,
        };
    }

    pub fn set_lowpass(&mut self, freq_hz: f32, q: f32, sample_rate: f32) {
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let a0 = 1.0 + alpha;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: ((1.0 - cos_omega) / 2.0) * a0_inv,
            b1: (1.0 - cos_omega) * a0_inv,
            b2: ((1.0 - cos_omega) / 2.0) * a0_inv,
            a1: (-2.0 * cos_omega) * a0_inv,
            a2: (1.0 - alpha) * a0_inv,
        };
    }

    pub fn set_bandpass(&mut self, freq_hz: f32, q: f32, sample_rate: f32) {
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let a0 = 1.0 + alpha;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: alpha * a0_inv,
            b1: 0.0,
            b2: -alpha * a0_inv,
            a1: (-2.0 * cos_omega) * a0_inv,
            a2: (1.0 - alpha) * a0_inv,
        };
    }

    pub fn set_notch(&mut self, freq_hz: f32, q: f32, sample_rate: f32) {
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let a0 = 1.0 + alpha;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: a0_inv,
            b1: (-2.0 * cos_omega) * a0_inv,
            b2: a0_inv,
            a1: (-2.0 * cos_omega) * a0_inv,
            a2: (1.0 - alpha) * a0_inv,
        };
    }

    pub fn set_peak(&mut self, freq_hz: f32, q: f32, gain_db: f32, sample_rate: f32) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let a0 = 1.0 + alpha / a;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: (1.0 + alpha * a) * a0_inv,
            b1: (-2.0 * cos_omega) * a0_inv,
            b2: (1.0 - alpha * a) * a0_inv,
            a1: (-2.0 * cos_omega) * a0_inv,
            a2: (1.0 - alpha / a) * a0_inv,
        };
    }

    pub fn set_low_shelf(&mut self, freq_hz: f32, q: f32, gain_db: f32, sample_rate: f32) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);
        let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;

        let a0 = (a + 1.0) + (a - 1.0) * cos_omega + two_sqrt_a_alpha;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: (a * ((a + 1.0) - (a - 1.0) * cos_omega + two_sqrt_a_alpha)) * a0_inv,
            b1: (2.0 * a * ((a - 1.0) - (a + 1.0) * cos_omega)) * a0_inv,
            b2: (a * ((a + 1.0) - (a - 1.0) * cos_omega - two_sqrt_a_alpha)) * a0_inv,
            a1: (-2.0 * ((a - 1.0) + (a + 1.0) * cos_omega)) * a0_inv,
            a2: ((a + 1.0) + (a - 1.0) * cos_omega - two_sqrt_a_alpha) * a0_inv,
        };
    }

    pub fn set_high_shelf(&mut self, freq_hz: f32, q: f32, gain_db: f32, sample_rate: f32) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);
        let two_sqrt_a_alpha = 2.0 * a.sqrt() * alpha;

        let a0 = (a + 1.0) - (a - 1.0) * cos_omega + two_sqrt_a_alpha;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: (a * ((a + 1.0) + (a - 1.0) * cos_omega + two_sqrt_a_alpha)) * a0_inv,
            b1: (-2.0 * a * ((a - 1.0) + (a + 1.0) * cos_omega)) * a0_inv,
            b2: (a * ((a + 1.0) + (a - 1.0) * cos_omega - two_sqrt_a_alpha)) * a0_inv,
            a1: (2.0 * ((a - 1.0) - (a + 1.0) * cos_omega)) * a0_inv,
            a2: ((a + 1.0) - (a - 1.0) * cos_omega - two_sqrt_a_alpha) * a0_inv,
        };
    }

    pub fn set_allpass(&mut self, freq_hz: f32, q: f32, sample_rate: f32) {
        let omega = 2.0 * PI * freq_hz / sample_rate;
        let sin_omega = omega.sin();
        let cos_omega = omega.cos();
        let alpha = sin_omega / (2.0 * q);

        let a0 = 1.0 + alpha;
        let a0_inv = 1.0 / a0;

        self.coeffs = BiquadCoeffs {
            b0: (1.0 - alpha) * a0_inv,
            b1: (-2.0 * cos_omega) * a0_inv,
            b2: (1.0 + alpha) * a0_inv,
            a1: (-2.0 * cos_omega) * a0_inv,
            a2: (1.0 - alpha) * a0_inv,
        };
    }

    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        let c = &self.coeffs;
        let output = c.b0 * input + self.z1;
        self.z1 = c.b1 * input - c.a1 * output + self.z2;
        self.z2 = c.b2 * input - c.a2 * output;
        output
    }

    pub fn reset(&mut self) {
        self.z1 = 0.0;
        self.z2 = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highpass_passes_high_frequencies() {
        let mut filter = Biquad::new();
        filter.set_highpass(1000.0, 0.707, 44100.0);

        let mut output = 0.0_f32;
        for i in 0..4410 {
            let t = i as f32 / 44100.0;
            let input = (2.0 * PI * 5000.0 * t).sin();
            output = filter.process(input);
        }
        assert!(output.abs() > 0.1, "High frequency should pass through");
    }

    #[test]
    fn test_reset_clears_state() {
        let mut filter = Biquad::new();
        filter.set_highpass(1000.0, 0.707, 44100.0);
        filter.process(1.0);
        filter.reset();
        assert_eq!(filter.z1, 0.0);
        assert_eq!(filter.z2, 0.0);
    }
}
