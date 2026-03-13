use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisSynth;

nih_export_clap!(OasisSynth);
nih_export_vst3!(OasisSynth);
