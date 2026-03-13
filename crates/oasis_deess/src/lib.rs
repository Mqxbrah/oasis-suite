use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisDeEss;

nih_export_clap!(OasisDeEss);
nih_export_vst3!(OasisDeEss);
