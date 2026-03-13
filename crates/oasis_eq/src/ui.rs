use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaTheming};
use oasis_ui::{ArrowButton, ArrowDirection, DropdownOverlay, ParamKnob};
use std::sync::Arc;

use crate::params::OasisEqParams;
use crate::presets;

#[derive(Lens, Clone)]
struct Data {
    params: Arc<OasisEqParams>,
    preset_name: String,
    show_preset_list: bool,
}

#[derive(Debug, Clone)]
enum PresetAction {
    Next,
    Previous,
    Select(usize),
    ToggleList,
}

fn band_param_ptr(params: &OasisEqParams, param_id: &str) -> Option<ParamPtr> {
    if param_id == "output_gain" {
        return Some(params.output_gain.as_ptr());
    }

    if param_id.len() < 4 || !param_id.starts_with('b') {
        return None;
    }

    let band_char = param_id.as_bytes()[1];
    if !(b'1'..=b'8').contains(&band_char) {
        return None;
    }
    let band_idx = (band_char - b'1') as usize;

    let suffix = &param_id[3..];
    match suffix {
        "freq" => Some(params.bands[band_idx].freq.as_ptr()),
        "gain" => Some(params.bands[band_idx].gain.as_ptr()),
        "q" => Some(params.bands[band_idx].q.as_ptr()),
        "type" => Some(params.bands[band_idx].filter_type.as_ptr()),
        "on" => Some(params.bands[band_idx].enabled.as_ptr()),
        _ => None,
    }
}

impl Model for Data {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|action, _| {
            let new_idx = match action {
                PresetAction::Next => {
                    presets::next_preset();
                    Some(
                        presets::CURRENT_PRESET_INDEX
                            .load(std::sync::atomic::Ordering::Relaxed),
                    )
                }
                PresetAction::Previous => {
                    presets::prev_preset();
                    Some(
                        presets::CURRENT_PRESET_INDEX
                            .load(std::sync::atomic::Ordering::Relaxed),
                    )
                }
                PresetAction::Select(idx) => Some(*idx),
                PresetAction::ToggleList => {
                    self.show_preset_list = !self.show_preset_list;
                    None
                }
            };

            if let Some(idx) = new_idx {
                if idx < presets::FACTORY_PRESETS.len() {
                    presets::CURRENT_PRESET_INDEX
                        .store(idx, std::sync::atomic::Ordering::Relaxed);
                    let preset = &presets::FACTORY_PRESETS[idx];

                    let updates: Vec<(ParamPtr, f32)> = preset
                        .values
                        .iter()
                        .filter_map(|&(param_id, norm_val)| {
                            band_param_ptr(&self.params, param_id).map(|p| (p, norm_val))
                        })
                        .collect();

                    for (ptr, norm_val) in updates {
                        cx.emit(RawParamEvent::BeginSetParameter(ptr));
                        cx.emit(RawParamEvent::SetParameterNormalized(ptr, norm_val));
                        cx.emit(RawParamEvent::EndSetParameter(ptr));
                    }

                    self.preset_name = presets::current_preset_name().to_string();
                    self.show_preset_list = false;
                }
            }
        });
    }
}

fn knob_control<L, Params, P, FMap>(
    cx: &mut Context,
    label_text: &str,
    params: L,
    params_to_param: FMap,
    bipolar: bool,
) where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: Param + 'static,
    FMap: Fn(&Params) -> &P + Copy + 'static,
{
    let value_lens = params.clone().map(move |p| {
        let param = params_to_param(p);
        param.normalized_value_to_string(param.unmodulated_normalized_value(), true)
    });

    VStack::new(cx, |cx| {
        Label::new(cx, label_text).class("knob-label");
        ParamKnob::new(cx, params, params_to_param, bipolar);
        Label::new(cx, value_lens).class("knob-value");
    })
    .class("knob-group");
}

fn toggle_control<L, Params, P, FMap>(
    cx: &mut Context,
    label_text: &str,
    params: L,
    params_to_param: FMap,
) where
    L: Lens<Target = Params> + Clone,
    Params: 'static,
    P: Param + 'static,
    FMap: Fn(&Params) -> &P + Copy + 'static,
{
    HStack::new(cx, |cx| {
        Label::new(cx, label_text).class("toggle-label");
        ParamButton::new(cx, params, params_to_param).class("toggle-btn-wrap");
    })
    .class("toggle-row");
}

fn band_column(cx: &mut Context, idx: usize) {
    VStack::new(cx, move |cx| {
        Label::new(cx, &format!("BAND {}", idx + 1)).class("section-title");

        knob_control(
            cx,
            "Freq",
            Data::params,
            move |p: &Arc<OasisEqParams>| &p.bands[idx].freq,
            false,
        );
        knob_control(
            cx,
            "Gain",
            Data::params,
            move |p: &Arc<OasisEqParams>| &p.bands[idx].gain,
            true,
        );
        knob_control(
            cx,
            "Q",
            Data::params,
            move |p: &Arc<OasisEqParams>| &p.bands[idx].q,
            false,
        );

        toggle_control(
            cx,
            "Type",
            Data::params,
            move |p: &Arc<OasisEqParams>| &p.bands[idx].filter_type,
        );
        toggle_control(
            cx,
            "On",
            Data::params,
            move |p: &Arc<OasisEqParams>| &p.bands[idx].enabled,
        );
    })
    .class("band-column");
}

pub fn create_editor(params: Arc<OasisEqParams>) -> Option<Box<dyn Editor>> {
    let editor_state = params.editor_state.clone();

    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        cx.add_stylesheet(oasis_ui::stylesheet())
            .expect("Failed to load stylesheet");

        Data {
            params: params.clone(),
            preset_name: presets::current_preset_name().to_string(),
            show_preset_list: false,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // Header
            HStack::new(cx, |cx| {
                Label::new(cx, "OASIS EQ")
                    .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                    .font_weight(FontWeightKeyword::Bold)
                    .class("header-title");

                Label::new(cx, "v1.0").class("header-version");

                HStack::new(cx, |cx| {
                    ArrowButton::new(cx, ArrowDirection::Left, PresetAction::Previous);

                    Label::new(cx, Data::preset_name)
                        .class("preset-name")
                        .on_press(|cx| cx.emit(PresetAction::ToggleList));

                    ArrowButton::new(cx, ArrowDirection::Right, PresetAction::Next);
                })
                .class("preset-browser");
            })
            .class("header-bar");

            // Bands 1-4
            HStack::new(cx, |cx| {
                band_column(cx, 0);
                band_column(cx, 1);
                band_column(cx, 2);
                band_column(cx, 3);
            })
            .class("band-row");

            // Bands 5-8
            HStack::new(cx, |cx| {
                band_column(cx, 4);
                band_column(cx, 5);
                band_column(cx, 6);
                band_column(cx, 7);
            })
            .class("band-row");

            // Output section
            VStack::new(cx, |cx| {
                Label::new(cx, "OUTPUT").class("section-title");
                HStack::new(cx, |cx| {
                    knob_control(cx, "Gain", Data::params, |p| &p.output_gain, true);
                })
                .class("knob-row");
            })
            .class("section");

            // Footer
            HStack::new(cx, |cx| {
                Label::new(cx, "Oasis Suite").class("footer-text");
            })
            .class("footer");
        })
        .class("main-container");

        DropdownOverlay::new(cx, Data::show_preset_list, |cx| {
            for (i, preset) in presets::FACTORY_PRESETS.iter().enumerate() {
                let idx = i;
                Label::new(cx, preset.name)
                    .class("preset-list-item")
                    .on_press(move |cx| {
                        cx.emit(PresetAction::Select(idx));
                    });
            }
        });
    })
}
