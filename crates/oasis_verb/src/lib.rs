use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisVerb;

nih_export_clap!(OasisVerb);
nih_export_vst3!(OasisVerb);
