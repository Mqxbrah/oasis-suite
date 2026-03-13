use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisWide;

nih_export_clap!(OasisWide);
nih_export_vst3!(OasisWide);
