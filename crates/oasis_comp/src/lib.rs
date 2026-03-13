use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisComp;

nih_export_clap!(OasisComp);
nih_export_vst3!(OasisComp);
