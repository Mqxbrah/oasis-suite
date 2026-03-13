use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisPunch;

nih_export_clap!(OasisPunch);
nih_export_vst3!(OasisPunch);
