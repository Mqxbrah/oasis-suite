use oasis_core::constants::*;
use oasis_core::util::denormal::sanitize;
use oasis_core::util::math;
use oasis_core::util::sample_rate::SampleRateContext;

use crate::params::ShiftMode;

const BUFFER_SIZE: usize = SHIFT_GRAIN_SIZE * SHIFT_OVERLAP;
const DETUNE_PAN_SPREAD: f32 = 0.6;

/// Single-channel granular pitch shifter with two overlapping grains.
struct GrainShifter {
    buffer: Vec<f32>,
    write_pos: usize,
    read_pos_a: f32,
    read_pos_b: f32,
    grain_counter_a: usize,
    grain_counter_b: usize,
}

impl GrainShifter {
    fn new() -> Self {
        Self {
            buffer: vec![0.0; BUFFER_SIZE],
            write_pos: 0,
            read_pos_a: 0.0,
            read_pos_b: 0.0,
            grain_counter_a: 0,
            grain_counter_b: SHIFT_GRAIN_SIZE / 2,
        }
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.read_pos_a = 0.0;
        self.read_pos_b = 0.0;
        self.grain_counter_a = 0;
        self.grain_counter_b = SHIFT_GRAIN_SIZE / 2;
    }

    #[inline]
    fn process(&mut self, input: f32, pitch_ratio: f32) -> f32 {
        self.buffer[self.write_pos] = input;

        let out_a = self.read_interpolated(self.read_pos_a);
        let out_b = self.read_interpolated(self.read_pos_b);

        let phase_a = self.grain_counter_a as f32 / SHIFT_GRAIN_SIZE as f32;
        let phase_b = self.grain_counter_b as f32 / SHIFT_GRAIN_SIZE as f32;
        let window_a = hann(phase_a);
        let window_b = hann(phase_b);

        self.read_pos_a += pitch_ratio;
        self.read_pos_b += pitch_ratio;
        self.wrap_read_pos_a();
        self.wrap_read_pos_b();

        self.grain_counter_a += 1;
        self.grain_counter_b += 1;

        if self.grain_counter_a >= SHIFT_GRAIN_SIZE {
            self.grain_counter_a = 0;
            self.read_pos_a = self.write_pos as f32;
        }
        if self.grain_counter_b >= SHIFT_GRAIN_SIZE {
            self.grain_counter_b = 0;
            self.read_pos_b = self.write_pos as f32;
        }

        self.write_pos = (self.write_pos + 1) % BUFFER_SIZE;

        out_a * window_a + out_b * window_b
    }

    #[inline]
    fn read_interpolated(&self, pos: f32) -> f32 {
        let idx0 = pos as usize % BUFFER_SIZE;
        let idx1 = (idx0 + 1) % BUFFER_SIZE;
        let frac = pos - pos.floor();
        self.buffer[idx0] * (1.0 - frac) + self.buffer[idx1] * frac
    }

    #[inline]
    fn wrap_read_pos_a(&mut self) {
        let size = BUFFER_SIZE as f32;
        if self.read_pos_a >= size {
            self.read_pos_a -= size;
        } else if self.read_pos_a < 0.0 {
            self.read_pos_a += size;
        }
    }

    #[inline]
    fn wrap_read_pos_b(&mut self) {
        let size = BUFFER_SIZE as f32;
        if self.read_pos_b >= size {
            self.read_pos_b -= size;
        } else if self.read_pos_b < 0.0 {
            self.read_pos_b += size;
        }
    }
}

/// Raised cosine (Hann) window. `phase` in [0, 1].
#[inline]
fn hann(phase: f32) -> f32 {
    0.5 * (1.0 - (std::f32::consts::TAU * phase).cos())
}

pub struct ShiftProcessor {
    shifter_l: GrainShifter,
    shifter_r: GrainShifter,
    detune_shifter_l: GrainShifter,
    detune_shifter_r: GrainShifter,
    _sample_rate: f32,
}

impl ShiftProcessor {
    pub fn new() -> Self {
        Self {
            shifter_l: GrainShifter::new(),
            shifter_r: GrainShifter::new(),
            detune_shifter_l: GrainShifter::new(),
            detune_shifter_r: GrainShifter::new(),
            _sample_rate: DEFAULT_SAMPLE_RATE,
        }
    }

    pub fn set_sample_rate(&mut self, ctx: &SampleRateContext) {
        self._sample_rate = ctx.sample_rate;
    }

    pub fn reset(&mut self) {
        self.shifter_l.reset();
        self.shifter_r.reset();
        self.detune_shifter_l.reset();
        self.detune_shifter_r.reset();
    }

    #[inline]
    pub fn process_sample(
        &mut self,
        left: f32,
        right: f32,
        semitones: f32,
        cents: f32,
        detune: f32,
        mode: ShiftMode,
        mix: f32,
        output_gain_db: f32,
    ) -> (f32, f32) {
        let base_shift = semitones + cents / 100.0;
        let gain = math::db_to_gain(output_gain_db);

        match mode {
            ShiftMode::Simple => {
                let ratio = 2.0f32.powf(base_shift / 12.0);
                let wet_l = self.shifter_l.process(left, ratio);
                let wet_r = self.shifter_r.process(right, ratio);

                let out_l = math::lerp(left, wet_l, mix) * gain;
                let out_r = math::lerp(right, wet_r, mix) * gain;
                (sanitize(out_l), sanitize(out_r))
            }
            ShiftMode::Detune => {
                let ratio_up = 2.0f32.powf((base_shift + detune / 100.0) / 12.0);
                let ratio_down = 2.0f32.powf((base_shift - detune / 100.0) / 12.0);

                let v1_l = self.shifter_l.process(left, ratio_up);
                let v1_r = self.shifter_r.process(right, ratio_up);
                let v2_l = self.detune_shifter_l.process(left, ratio_down);
                let v2_r = self.detune_shifter_r.process(right, ratio_down);

                let near = DETUNE_PAN_SPREAD;
                let far = 1.0 - DETUNE_PAN_SPREAD;
                let wet_l = v1_l * near + v2_l * far;
                let wet_r = v1_r * far + v2_r * near;

                let out_l = math::lerp(left, wet_l, mix) * gain;
                let out_r = math::lerp(right, wet_r, mix) * gain;
                (sanitize(out_l), sanitize(out_r))
            }
        }
    }
}
