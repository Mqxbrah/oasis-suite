use crate::constants::DEFAULT_SAMPLE_RATE;

#[derive(Clone, Copy)]
pub struct SampleRateContext {
    pub sample_rate: f32,
    pub sample_rate_recip: f32,
    pub nyquist: f32,
}

impl SampleRateContext {
    pub fn new(sample_rate: f32) -> Self {
        let sr = if sample_rate > 0.0 { sample_rate } else { DEFAULT_SAMPLE_RATE };
        Self {
            sample_rate: sr,
            sample_rate_recip: 1.0 / sr,
            nyquist: sr * 0.5,
        }
    }

    pub fn ms_to_samples(&self, ms: f32) -> f32 {
        ms * 0.001 * self.sample_rate
    }

    pub fn hz_to_normalized(&self, hz: f32) -> f32 {
        hz * self.sample_rate_recip
    }
}
