use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisEq;

nih_export_clap!(OasisEq);
nih_export_vst3!(OasisEq);
