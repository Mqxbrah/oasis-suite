pub mod widgets;

pub use nih_plug_vizia;
pub use nih_plug_vizia::vizia;
pub use nih_plug_vizia::ViziaState;
pub use widgets::ParamKnob;

use std::sync::Arc;

pub fn default_editor_size() -> Arc<ViziaState> {
    ViziaState::new(|| (580, 420))
}

pub fn stylesheet() -> &'static str {
    include_str!("style.css")
}
