use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisPump;

nih_export_clap!(OasisPump);
nih_export_vst3!(OasisPump);
