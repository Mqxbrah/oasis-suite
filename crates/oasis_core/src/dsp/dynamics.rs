use crate::util::denormal::sanitize;

pub struct EnvelopeDetector {
    attack_coeff: f32,
    release_coeff: f32,
    envelope: f32,
}

impl EnvelopeDetector {
    pub fn new() -> Self {
        Self {
            attack_coeff: 0.0,
            release_coeff: 0.0,
            envelope: 0.0,
        }
    }

    pub fn set_times(&mut self, attack_ms: f32, release_ms: f32, sample_rate: f32) {
        self.attack_coeff = Self::time_constant(attack_ms, sample_rate);
        self.release_coeff = Self::time_constant(release_ms, sample_rate);
    }

    fn time_constant(ms: f32, sample_rate: f32) -> f32 {
        if ms <= 0.0 {
            return 1.0;
        }
        (-1.0 / (ms * 0.001 * sample_rate)).exp()
    }

    #[inline]
    pub fn process_peak(&mut self, input: f32) -> f32 {
        let level = input.abs();
        let coeff = if level > self.envelope {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        self.envelope = coeff * self.envelope + (1.0 - coeff) * level;
        sanitize(self.envelope)
    }

    #[inline]
    pub fn process_rms(&mut self, input: f32) -> f32 {
        let level = input * input;
        let coeff = if level > self.envelope * self.envelope {
            self.attack_coeff
        } else {
            self.release_coeff
        };
        self.envelope = coeff * self.envelope + (1.0 - coeff) * level.sqrt();
        sanitize(self.envelope)
    }

    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }

    pub fn current(&self) -> f32 {
        self.envelope
    }
}

pub struct GainComputer {
    pub threshold_db: f32,
    pub ratio: f32,
    pub knee_db: f32,
}

impl GainComputer {
    pub fn new(threshold_db: f32, ratio: f32, knee_db: f32) -> Self {
        Self {
            threshold_db,
            ratio,
            knee_db,
        }
    }

    #[inline]
    pub fn compute_gain_db(&self, input_db: f32) -> f32 {
        let half_knee = self.knee_db * 0.5;

        if input_db < self.threshold_db - half_knee {
            0.0
        } else if input_db > self.threshold_db + half_knee {
            (self.threshold_db - input_db) * (1.0 - 1.0 / self.ratio)
        } else {
            let x = input_db - self.threshold_db + half_knee;
            (1.0 - 1.0 / self.ratio) * x * x / (2.0 * self.knee_db).max(0.001)
                * -1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_attack() {
        let mut det = EnvelopeDetector::new();
        det.set_times(1.0, 100.0, 44100.0);
        for _ in 0..4410 {
            det.process_peak(1.0);
        }
        assert!(det.current() > 0.9, "Envelope should converge near 1.0");
    }

    #[test]
    fn test_gain_computer_below_threshold() {
        let gc = GainComputer::new(-20.0, 4.0, 0.0);
        let gain = gc.compute_gain_db(-30.0);
        assert!((gain - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_gain_computer_above_threshold() {
        let gc = GainComputer::new(-20.0, 4.0, 0.0);
        let gain = gc.compute_gain_db(-10.0);
        assert!(gain < 0.0, "Should reduce gain above threshold");
    }
}
