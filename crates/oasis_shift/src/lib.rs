use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisShift;

nih_export_clap!(OasisShift);
nih_export_vst3!(OasisShift);
