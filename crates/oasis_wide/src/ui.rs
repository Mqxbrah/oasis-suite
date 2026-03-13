use nih_plug::prelude::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaTheming};
use std::sync::Arc;

use crate::params::OasisWideParams;
use crate::presets;

#[derive(Lens, Clone)]
struct Data {
    params: Arc<OasisWideParams>,
}

impl Model for Data {}

#[derive(Debug)]
enum PresetAction {
    Next,
    Previous,
}

struct PresetBrowser;

impl PresetBrowser {
    fn new(cx: &mut Context) -> Handle<'_, Self> {
        Self.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |cx| cx.emit(PresetAction::Previous),
                    |cx| Label::new(cx, "\u{25C0}"),
                )
                .class("preset-nav-btn");

                Label::new(cx, "Init")
                    .class("preset-name")
                    .id("preset-label");

                Button::new(
                    cx,
                    |cx| cx.emit(PresetAction::Next),
                    |cx| Label::new(cx, "\u{25B6}"),
                )
                .class("preset-nav-btn");
            })
            .class("preset-browser");
        })
    }
}

impl View for PresetBrowser {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|action, _| {
            match action {
                PresetAction::Next => { presets::next_preset(); }
                PresetAction::Previous => { presets::prev_preset(); }
            }

            let idx = presets::CURRENT_PRESET_INDEX
                .load(std::sync::atomic::Ordering::Relaxed);
            if idx >= presets::FACTORY_PRESETS.len() {
                return;
            }
            let preset = &presets::FACTORY_PRESETS[idx];

            // Collect param pointers while borrowing data immutably,
            // then release the borrow before emitting events.
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
        });
    }
}

fn param_row<L, Params, P, FMap>(
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
        Label::new(cx, label_text).class("param-label");
        ParamSlider::new(cx, params, params_to_param)
            .class("param-slider-wrap")
            .set_style(ParamSliderStyle::FromLeft);
    })
    .class("param-row");
}

fn param_row_centered<L, Params, P, FMap>(
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
        Label::new(cx, label_text).class("param-label");
        ParamSlider::new(cx, params, params_to_param)
            .class("param-slider-wrap")
            .set_style(ParamSliderStyle::Centered);
    })
    .class("param-row");
}

fn param_row_button<L, Params, P, FMap>(
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
        Label::new(cx, label_text).class("param-label");
        ParamButton::new(cx, params, params_to_param)
            .class("param-slider-wrap");
    })
    .class("param-row");
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
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // ── Header Bar ──
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

            // ── Main Content ──
            HStack::new(cx, |cx| {
                // ── Left Column: Stereo Image ──
                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "STEREO IMAGE").class("section-title");

                        param_row(
                            cx, "Width",
                            Data::params, |p| &p.width,
                        );
                        param_row_centered(
                            cx, "Mid Gain",
                            Data::params, |p| &p.mid_gain,
                        );
                        param_row_centered(
                            cx, "Side Gain",
                            Data::params, |p| &p.side_gain,
                        );
                    })
                    .class("section");

                    VStack::new(cx, |cx| {
                        Label::new(cx, "BASS MONO").class("section-title");

                        param_row_button(
                            cx, "Enable",
                            Data::params, |p| &p.bass_mono_enabled,
                        );
                        param_row(
                            cx, "Frequency",
                            Data::params, |p| &p.bass_mono_freq,
                        );
                    })
                    .class("section");
                })
                .class("column");

                // ── Right Column: Haas + Output ──
                VStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "HAAS EFFECT").class("section-title");

                        param_row(
                            cx, "Delay",
                            Data::params, |p| &p.haas_delay_ms,
                        );
                        param_row_button(
                            cx, "Channel",
                            Data::params, |p| &p.haas_channel,
                        );
                    })
                    .class("section");

                    VStack::new(cx, |cx| {
                        Label::new(cx, "OUTPUT").class("section-title");

                        param_row(
                            cx, "Mix",
                            Data::params, |p| &p.mix,
                        );
                        param_row_centered(
                            cx, "Gain",
                            Data::params, |p| &p.output_gain,
                        );
                    })
                    .class("section");
                })
                .class("column");
            })
            .class("columns")
            .class("content-area");

            // ── Footer ──
            HStack::new(cx, |cx| {
                Label::new(cx, "Oasis Suite")
                    .class("footer-text");
            })
            .class("footer");
        })
        .class("main-container");
    })
}
