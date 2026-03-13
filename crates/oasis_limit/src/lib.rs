use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisLimit;

nih_export_clap!(OasisLimit);
nih_export_vst3!(OasisLimit);
