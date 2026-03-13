use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaTheming};
use oasis_ui::{ArrowButton, ArrowDirection, DropdownOverlay, ParamKnob};
use std::sync::Arc;

use crate::params::OasisWideParams;
use crate::presets;

#[derive(Lens, Clone)]
struct Data {
    params: Arc<OasisWideParams>,
    preset_name: String,
    show_preset_list: bool,
}

#[derive(Debug, Clone)]
enum DataEvent {
    PresetChanged,
    TogglePresetList,
    ClosePresetList,
}

impl Model for Data {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            DataEvent::PresetChanged => {
                self.preset_name = presets::current_preset_name().to_string();
                self.show_preset_list = false;
            }
            DataEvent::TogglePresetList => {
                self.show_preset_list = !self.show_preset_list;
            }
            DataEvent::ClosePresetList => {
                self.show_preset_list = false;
            }
        });
    }
}

#[derive(Debug, Clone)]
enum PresetAction {
    Next,
    Previous,
    Select(usize),
}

fn apply_preset(cx: &mut EventContext, idx: usize) {
    if idx >= presets::FACTORY_PRESETS.len() {
        return;
    }

    presets::CURRENT_PRESET_INDEX.store(idx, std::sync::atomic::Ordering::Relaxed);
    let preset = &presets::FACTORY_PRESETS[idx];

    let updates: Vec<(ParamPtr, f32)> = if let Some(data) = cx.data::<Data>() {
        let params = &data.params;
        preset.values.iter().filter_map(|&(param_id, norm_val)| {
            let ptr = match param_id {
                "width" => Some(params.width.as_ptr()),
                "mid_gain" => Some(params.mid_gain.as_ptr()),
                "side_gain" => Some(params.side_gain.as_ptr()),
                "haas_delay" => Some(params.haas_delay_ms.as_ptr()),
                "haas_channel" => Some(params.haas_channel.as_ptr()),
                "bass_mono_on" => Some(params.bass_mono_enabled.as_ptr()),
                "bass_mono_freq" => Some(params.bass_mono_freq.as_ptr()),
                "mix" => Some(params.mix.as_ptr()),
                "output_gain" => Some(params.output_gain.as_ptr()),
                _ => None,
            };
            ptr.map(|p| (p, norm_val))
        }).collect()
    } else {
        return;
    };

    for (ptr, norm_val) in updates {
        cx.emit(RawParamEvent::BeginSetParameter(ptr));
        cx.emit(RawParamEvent::SetParameterNormalized(ptr, norm_val));
        cx.emit(RawParamEvent::EndSetParameter(ptr));
    }

    cx.emit(DataEvent::PresetChanged);
}

struct PresetBrowser;

impl PresetBrowser {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self.build(cx, |cx| {
            HStack::new(cx, |cx| {
                ArrowButton::new(cx, ArrowDirection::Left, PresetAction::Previous);

                VStack::new(cx, |cx| {
                    Label::new(cx, Data::preset_name)
                        .class("preset-name")
                        .on_press(|cx| cx.emit(DataEvent::TogglePresetList));

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
                .class("preset-name-container");

                ArrowButton::new(cx, ArrowDirection::Right, PresetAction::Next);
            })
            .class("preset-browser");
        })
    }
}

impl View for PresetBrowser {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|action, _| {
            match action {
                PresetAction::Next => {
                    presets::next_preset();
                    let idx = presets::CURRENT_PRESET_INDEX
                        .load(std::sync::atomic::Ordering::Relaxed);
                    apply_preset(cx, idx);
                }
                PresetAction::Previous => {
                    presets::prev_preset();
                    let idx = presets::CURRENT_PRESET_INDEX
                        .load(std::sync::atomic::Ordering::Relaxed);
                    apply_preset(cx, idx);
                }
                PresetAction::Select(idx) => {
                    apply_preset(cx, *idx);
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

pub fn create_editor(params: Arc<OasisWideParams>) -> Option<Box<dyn Editor>> {
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
                Label::new(cx, "OASIS WIDE")
                    .font_family(vec![FamilyOwned::Name(String::from(
                        assets::NOTO_SANS,
                    ))])
                    .font_weight(FontWeightKeyword::Bold)
                    .class("header-title");

                Label::new(cx, "v1.0")
                    .class("header-version");

                PresetBrowser::new(cx);
            })
            .class("header-bar");

            HStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "STEREO IMAGE").class("section-title");
                        HStack::new(cx, |cx| {
                            knob_control(cx, "Width", Data::params, |p| &p.width, false);
                            knob_control(cx, "Mid", Data::params, |p| &p.mid_gain, true);
                            knob_control(cx, "Side", Data::params, |p| &p.side_gain, true);
                        })
                        .class("knob-row");
                    })
                    .class("section");

                    VStack::new(cx, |cx| {
                        Label::new(cx, "BASS MONO").class("section-title");
                        toggle_control(cx, "Enable", Data::params, |p| &p.bass_mono_enabled);
                        HStack::new(cx, |cx| {
                            knob_control(cx, "Frequency", Data::params, |p| &p.bass_mono_freq, false);
                        })
                        .class("knob-row");
                    })
                    .class("section");
                })
                .class("column");

                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "HAAS EFFECT").class("section-title");
                        toggle_control(cx, "Channel", Data::params, |p| &p.haas_channel);
                        HStack::new(cx, |cx| {
                            knob_control(cx, "Delay", Data::params, |p| &p.haas_delay_ms, false);
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
    })
}
