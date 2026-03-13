use oasis_core::dsp::filter::Biquad;
use oasis_core::util::denormal::sanitize;

use crate::params::FilterType;

pub struct EqProcessor {
    filters_l: [Biquad; 8],
    filters_r: [Biquad; 8],
    sample_rate: f32,

    current_freq: [f32; 8],
    current_gain: [f32; 8],
    current_q: [f32; 8],
    current_type: [FilterType; 8],
}

impl EqProcessor {
    pub fn new() -> Self {
        Self {
            filters_l: std::array::from_fn(|_| Biquad::new()),
            filters_r: std::array::from_fn(|_| Biquad::new()),
            sample_rate: 44100.0,
            current_freq: [0.0; 8],
            current_gain: [f32::MIN; 8],
            current_q: [0.0; 8],
            current_type: [FilterType::Bell; 8],
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.current_freq = [0.0; 8];
        self.current_gain = [f32::MIN; 8];
    }

    pub fn reset(&mut self) {
        for f in &mut self.filters_l {
            f.reset();
        }
        for f in &mut self.filters_r {
            f.reset();
        }
    }

    pub fn update_band(
        &mut self,
        band: usize,
        freq: f32,
        gain_db: f32,
        q: f32,
        filter_type: FilterType,
        enabled: bool,
    ) {
        if !enabled {
            return;
        }

        let freq_changed = (freq - self.current_freq[band]).abs() > 0.01;
        let gain_changed = (gain_db - self.current_gain[band]).abs() > 0.001;
        let q_changed = (q - self.current_q[band]).abs() > 0.001;
        let type_changed = filter_type != self.current_type[band];

        if !freq_changed && !gain_changed && !q_changed && !type_changed {
            return;
        }

        let sr = self.sample_rate;
        match filter_type {
            FilterType::Bell => {
                self.filters_l[band].set_peak(freq, q, gain_db, sr);
                self.filters_r[band].set_peak(freq, q, gain_db, sr);
            }
            FilterType::LowShelf => {
                self.filters_l[band].set_low_shelf(freq, q, gain_db, sr);
                self.filters_r[band].set_low_shelf(freq, q, gain_db, sr);
            }
            FilterType::HighShelf => {
                self.filters_l[band].set_high_shelf(freq, q, gain_db, sr);
                self.filters_r[band].set_high_shelf(freq, q, gain_db, sr);
            }
            FilterType::LowCut => {
                self.filters_l[band].set_highpass(freq, q, sr);
                self.filters_r[band].set_highpass(freq, q, sr);
            }
            FilterType::HighCut => {
                self.filters_l[band].set_lowpass(freq, q, sr);
                self.filters_r[band].set_lowpass(freq, q, sr);
            }
            FilterType::Notch => {
                self.filters_l[band].set_notch(freq, q, sr);
                self.filters_r[band].set_notch(freq, q, sr);
            }
            FilterType::Bandpass => {
                self.filters_l[band].set_bandpass(freq, q, sr);
                self.filters_r[band].set_bandpass(freq, q, sr);
            }
        }

        self.current_freq[band] = freq;
        self.current_gain[band] = gain_db;
        self.current_q[band] = q;
        self.current_type[band] = filter_type;
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left: f32,
        right: f32,
        band_enabled: &[bool; 8],
    ) -> (f32, f32) {
        let mut l = left;
        let mut r = right;

        for i in 0..8 {
            if band_enabled[i] {
                l = self.filters_l[i].process(l);
                r = self.filters_r[i].process(r);
            }
        }

        (sanitize(l), sanitize(r))
    }
}
