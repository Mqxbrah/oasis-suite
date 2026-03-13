pub struct DelayLine {
    buffer: Vec<f32>,
    write_pos: usize,
    max_delay_samples: usize,
}

impl DelayLine {
    pub fn new(max_delay_samples: usize) -> Self {
        Self {
            buffer: vec![0.0; max_delay_samples.max(1)],
            write_pos: 0,
            max_delay_samples: max_delay_samples.max(1),
        }
    }

    pub fn write(&mut self, sample: f32) {
        self.buffer[self.write_pos] = sample;
        self.write_pos = (self.write_pos + 1) % self.max_delay_samples;
    }

    /// Read from the delay line with linear interpolation.
    /// delay_samples=1 returns the most recently written sample,
    /// delay_samples=N returns the sample written N calls ago.
    #[inline]
    pub fn read_linear(&self, delay_samples: f32) -> f32 {
        if delay_samples < 0.5 {
            return self.buffer[(self.write_pos + self.max_delay_samples - 1) % self.max_delay_samples];
        }

        let delay_clamped = delay_samples.min((self.max_delay_samples - 1) as f32);
        let delay_int = delay_clamped as usize;
        let frac = delay_clamped - delay_int as f32;

        let idx0 = (self.write_pos + self.max_delay_samples - delay_int) % self.max_delay_samples;
        let idx1 = (idx0 + self.max_delay_samples - 1) % self.max_delay_samples;

        self.buffer[idx0] * (1.0 - frac) + self.buffer[idx1] * frac
    }

    pub fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }

    pub fn resize(&mut self, new_max: usize) {
        let new_max = new_max.max(1);
        if new_max != self.max_delay_samples {
            self.buffer.resize(new_max, 0.0);
            self.buffer.fill(0.0);
            self.max_delay_samples = new_max;
            self.write_pos = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_basic() {
        let mut dl = DelayLine::new(8);
        dl.write(1.0);
        dl.write(0.0);
        dl.write(0.0);
        let out = dl.read_linear(3.0);
        assert!((out - 1.0).abs() < 1e-6, "Expected 1.0, got {out}");
    }

    #[test]
    fn test_delay_reset() {
        let mut dl = DelayLine::new(4);
        dl.write(1.0);
        dl.reset();
        let out = dl.read_linear(1.0);
        assert!((out).abs() < 1e-6);
    }
}
