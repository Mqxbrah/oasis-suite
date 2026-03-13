use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaTheming};
use oasis_ui::{ArrowButton, ArrowDirection, DropdownOverlay, ParamKnob};
use std::sync::Arc;

use crate::params::OasisDelayParams;
use crate::presets;

#[derive(Lens, Clone)]
struct Data {
    params: Arc<OasisDelayParams>,
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

impl Model for Data {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|action, _| {
            let new_idx = match action {
                PresetAction::Next => {
                    presets::next_preset();
                    Some(presets::CURRENT_PRESET_INDEX
                        .load(std::sync::atomic::Ordering::Relaxed))
                }
                PresetAction::Previous => {
                    presets::prev_preset();
                    Some(presets::CURRENT_PRESET_INDEX
                        .load(std::sync::atomic::Ordering::Relaxed))
                }
                PresetAction::Select(idx) => {
                    Some(*idx)
                }
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

                    let updates: Vec<(ParamPtr, f32)> = {
                        let params = &self.params;
                        preset.values.iter().filter_map(|&(param_id, norm_val)| {
                            let ptr = match param_id {
                                "delay_l" => Some(params.delay_left_ms.as_ptr()),
                                "delay_r" => Some(params.delay_right_ms.as_ptr()),
                                "feedback" => Some(params.feedback.as_ptr()),
                                "ping_pong" => Some(params.ping_pong.as_ptr()),
                                "lp_freq" => Some(params.lp_freq.as_ptr()),
                                "hp_freq" => Some(params.hp_freq.as_ptr()),
                                "saturation" => Some(params.saturation.as_ptr()),
                                "mod_rate" => Some(params.mod_rate.as_ptr()),
                                "mod_depth" => Some(params.mod_depth.as_ptr()),
                                "mix" => Some(params.mix.as_ptr()),
                                "output_gain" => Some(params.output_gain.as_ptr()),
                                _ => None,
                            };
                            ptr.map(|p| (p, norm_val))
                        }).collect()
                    };

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
        ParamButton::new(cx, params, params_to_param)
            .class("toggle-btn-wrap");
    })
    .class("toggle-row");
}

pub fn create_editor(params: Arc<OasisDelayParams>) -> Option<Box<dyn Editor>> {
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
            HStack::new(cx, |cx| {
                Label::new(cx, "OASIS DELAY")
                    .font_family(vec![FamilyOwned::Name(String::from(
                        assets::NOTO_SANS,
                    ))])
                    .font_weight(FontWeightKeyword::Bold)
                    .class("header-title");

                Label::new(cx, "v1.0")
                    .class("header-version");

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

            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "TIME").class("section-title");
                        HStack::new(cx, |cx| {
                            knob_control(cx, "Delay L", Data::params, |p| &p.delay_left_ms, false);
                            knob_control(cx, "Delay R", Data::params, |p| &p.delay_right_ms, false);
                            knob_control(cx, "Feedback", Data::params, |p| &p.feedback, false);
                        })
                        .class("knob-row");
                        toggle_control(cx, "Ping-Pong", Data::params, |p| &p.ping_pong);
                    })
                    .class("section");
                })
                .class("column");

                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "COLOR").class("section-title");
                        HStack::new(cx, |cx| {
                            knob_control(cx, "LP", Data::params, |p| &p.lp_freq, false);
                            knob_control(cx, "HP", Data::params, |p| &p.hp_freq, false);
                            knob_control(cx, "Saturation", Data::params, |p| &p.saturation, false);
                        })
                        .class("knob-row");
                        HStack::new(cx, |cx| {
                            knob_control(cx, "Mod Rate", Data::params, |p| &p.mod_rate, false);
                            knob_control(cx, "Mod Depth", Data::params, |p| &p.mod_depth, false);
                        })
                        .class("knob-row");
                    })
                    .class("section");

                    VStack::new(cx, |cx| {
                        Label::new(cx, "OUTPUT").class("section-title");
                        HStack::new(cx, |cx| {
                            knob_control(cx, "Mix", Data::params, |p| &p.mix, false);
                            knob_control(cx, "Gain", Data::params, |p| &p.output_gain, true);
                        })
                        .class("knob-row");
                    })
                    .class("section");
                })
                .class("column");
            })
            .class("columns")
            .class("content-area");

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
