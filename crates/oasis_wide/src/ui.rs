use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaTheming};
use std::sync::Arc;

use crate::params::OasisWideParams;

#[derive(Lens)]
struct Data {
    params: Arc<OasisWideParams>,
}

impl Model for Data {}

pub fn create_editor(params: Arc<OasisWideParams>) -> Option<Box<dyn Editor>> {
    let editor_state = params.editor_state.clone();

    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        cx.add_stylesheet(oasis_ui::stylesheet())
            .expect("Failed to load stylesheet");

        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "OASIS WIDE")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(24.0)
                .height(Pixels(44.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(1.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0));

            ScrollView::new(cx, 0.0, 0.0, false, true, |cx| {
                GenericUi::new(cx, Data::params)
                    .child_top(Pixels(0.0));
            })
            .width(Percentage(100.0))
            .top(Pixels(5.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        ResizeHandle::new(cx);
    })
}
