pub use nih_plug_vizia;
pub use nih_plug_vizia::vizia;
pub use nih_plug_vizia::ViziaState;

use std::sync::Arc;

pub fn default_editor_size() -> Arc<ViziaState> {
    ViziaState::new(|| (560, 380))
}

pub fn stylesheet() -> &'static str {
    include_str!("style.css")
}
