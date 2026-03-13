use nih_plug::prelude::*;

mod dsp;
mod params;
mod plugin;
mod presets;
mod ui;

pub use plugin::OasisDrive;

nih_export_clap!(OasisDrive);
nih_export_vst3!(OasisDrive);
