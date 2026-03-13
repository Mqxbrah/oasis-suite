use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisDelay;

nih_export_clap!(OasisDelay);
nih_export_vst3!(OasisDelay);
