use nih_plug::prelude::Param;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
use nih_plug_vizia::widgets::util::ModifiersExt;
use nih_plug_vizia::widgets::RawParamEvent;
use std::f32::consts::PI;

const ARC_START: f32 = PI * 0.75;
const ARC_END: f32 = PI * 2.25;
const DRAG_SCALAR: f32 = 0.004;
const FINE_DRAG_SCALAR: f32 = 0.0004;

#[derive(Lens)]
pub struct ParamKnob {
    param_ptr: nih_plug::prelude::ParamPtr,
    is_dragging: bool,
    drag_start_y: f32,
    drag_start_value: f32,
    bipolar: bool,

    show_menu: bool,
    text_editing: bool,
}

enum KnobEvent {
    DragStart(f32, f32),
    DragMove(f32, bool),
    DragEnd,
    ResetToDefault,
    OpenMenu,
    CloseMenu,
    MenuEditValue,
    MenuCopy,
    MenuPaste,
    TextSubmit(String),
    TextCancel,
}

impl ParamKnob {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
        bipolar: bool,
    ) -> Handle<'_, Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        let param_ptr = params
            .clone()
            .map(move |params| params_to_param(params).as_ptr())
            .get(cx);

        Self {
            param_ptr,
            is_dragging: false,
            drag_start_y: 0.0,
            drag_start_value: 0.0,
            bipolar,
            show_menu: false,
            text_editing: false,
        }
        .build(cx, move |cx| {
            // Text input overlay (shown when editing value)
            Binding::new(cx, ParamKnob::text_editing, move |cx, editing| {
                if editing.get(cx) {
                    let display_lens = params.clone().map(move |p| {
                        let param = params_to_param(p);
                        param.normalized_value_to_string(
                            param.unmodulated_normalized_value(),
                            true,
                        )
                    });

                    Textbox::new(cx, display_lens)
                        .class("knob-text-input")
                        .on_submit(|cx, val, success| {
                            if success {
                                cx.emit(KnobEvent::TextSubmit(val));
                            } else {
                                cx.emit(KnobEvent::TextCancel);
                            }
                        })
                        .on_cancel(|cx| cx.emit(KnobEvent::TextCancel))
                        .on_build(|cx| {
                            cx.emit(TextEvent::StartEdit);
                            cx.emit(TextEvent::SelectAll);
                        })
                        .position_type(PositionType::SelfDirected)
                        .width(Pixels(80.0))
                        .height(Pixels(20.0))
                        .left(Stretch(1.0))
                        .right(Stretch(1.0))
                        .top(Stretch(1.0))
                        .bottom(Stretch(1.0))
                        .z_index(200);
                }
            });

            // Context menu popup
            Popup::new(cx, ParamKnob::show_menu, false, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Edit Value")
                        .class("menu-item")
                        .on_press(|cx| cx.emit(KnobEvent::MenuEditValue));
                    Label::new(cx, "Copy")
                        .class("menu-item")
                        .on_press(|cx| cx.emit(KnobEvent::MenuCopy));
                    Label::new(cx, "Paste")
                        .class("menu-item")
                        .on_press(|cx| cx.emit(KnobEvent::MenuPaste));
                    Label::new(cx, "Reset")
                        .class("menu-item")
                        .on_press(|cx| cx.emit(KnobEvent::ResetToDefault));
                })
                .class("context-menu");
            })
            .on_blur(|cx| cx.emit(KnobEvent::CloseMenu))
            .class("knob-popup");
        })
    }
}

impl View for ParamKnob {
    fn element(&self) -> Option<&'static str> {
        Some("param-knob")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match *window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if self.show_menu || self.text_editing {
                    return;
                }
                if cx.modifiers().alt() {
                    cx.emit(KnobEvent::ResetToDefault);
                } else {
                    let norm = unsafe { self.param_ptr.unmodulated_normalized_value() };
                    cx.emit(KnobEvent::DragStart(cx.mouse().cursory, norm));
                }
                meta.consume();
            }
            WindowEvent::MouseDoubleClick(MouseButton::Left) => {
                if self.show_menu || self.text_editing {
                    return;
                }
                cx.emit(KnobEvent::ResetToDefault);
                meta.consume();
            }
            WindowEvent::MouseUp(MouseButton::Left) => {
                if self.is_dragging {
                    cx.emit(KnobEvent::DragEnd);
                }
                meta.consume();
            }
            WindowEvent::MouseMove(_, y) => {
                if self.is_dragging {
                    let fine = cx.modifiers().shift();
                    cx.emit(KnobEvent::DragMove(y, fine));
                }
            }
            WindowEvent::MouseDown(MouseButton::Right) => {
                cx.emit(KnobEvent::OpenMenu);
                meta.consume();
            }
            _ => {}
        });

        event.map(|knob_event, _| match knob_event {
            KnobEvent::DragStart(y, norm) => {
                cx.capture();
                cx.set_active(true);
                self.is_dragging = true;
                self.drag_start_y = *y;
                self.drag_start_value = *norm;
                cx.emit(RawParamEvent::BeginSetParameter(self.param_ptr));
            }
            KnobEvent::DragMove(y, fine) => {
                let delta_y = self.drag_start_y - y;
                let scalar = if *fine { FINE_DRAG_SCALAR } else { DRAG_SCALAR };
                let new_value = (self.drag_start_value + delta_y * scalar).clamp(0.0, 1.0);
                cx.emit(RawParamEvent::SetParameterNormalized(
                    self.param_ptr,
                    new_value,
                ));
                cx.needs_redraw();
            }
            KnobEvent::DragEnd => {
                cx.release();
                cx.set_active(false);
                self.is_dragging = false;
                cx.emit(RawParamEvent::EndSetParameter(self.param_ptr));
            }
            KnobEvent::ResetToDefault => {
                self.show_menu = false;
                let default = unsafe { self.param_ptr.default_normalized_value() };
                cx.emit(RawParamEvent::BeginSetParameter(self.param_ptr));
                cx.emit(RawParamEvent::SetParameterNormalized(self.param_ptr, default));
                cx.emit(RawParamEvent::EndSetParameter(self.param_ptr));
                cx.needs_redraw();
            }
            KnobEvent::OpenMenu => {
                self.show_menu = true;
                cx.needs_redraw();
            }
            KnobEvent::CloseMenu => {
                self.show_menu = false;
                cx.needs_redraw();
            }
            KnobEvent::MenuEditValue => {
                self.show_menu = false;
                self.text_editing = true;
                cx.needs_redraw();
            }
            KnobEvent::MenuCopy => {
                self.show_menu = false;
                let value_str = unsafe {
                    self.param_ptr.normalized_value_to_string(
                        self.param_ptr.unmodulated_normalized_value(),
                        true,
                    )
                };
                let _ = cx.set_clipboard(value_str);
                cx.needs_redraw();
            }
            KnobEvent::MenuPaste => {
                self.show_menu = false;
                if let Ok(text) = cx.get_clipboard() {
                    if let Some(norm) = unsafe { self.param_ptr.string_to_normalized_value(&text) } {
                        cx.emit(RawParamEvent::BeginSetParameter(self.param_ptr));
                        cx.emit(RawParamEvent::SetParameterNormalized(self.param_ptr, norm));
                        cx.emit(RawParamEvent::EndSetParameter(self.param_ptr));
                    }
                }
                cx.needs_redraw();
            }
            KnobEvent::TextSubmit(text) => {
                self.text_editing = false;
                if let Some(norm) = unsafe { self.param_ptr.string_to_normalized_value(text) } {
                    cx.emit(RawParamEvent::BeginSetParameter(self.param_ptr));
                    cx.emit(RawParamEvent::SetParameterNormalized(self.param_ptr, norm));
                    cx.emit(RawParamEvent::EndSetParameter(self.param_ptr));
                }
                cx.needs_redraw();
            }
            KnobEvent::TextCancel => {
                self.text_editing = false;
                cx.needs_redraw();
            }
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let size = bounds.w.min(bounds.h);
        let center_x = bounds.x + bounds.w * 0.5;
        let center_y = bounds.y + bounds.h * 0.5;
        let radius = size * 0.42;
        let track_width = (size * 0.06).max(2.0);
        let value_width = (size * 0.08).max(2.5);

        let normalized = unsafe { self.param_ptr.unmodulated_normalized_value() };
        let value_angle = ARC_START + normalized * (ARC_END - ARC_START);

        // Track arc
        let mut path = vg::Path::new();
        path.arc(center_x, center_y, radius, ARC_START, ARC_END, vg::Solidity::Hole);
        let mut paint = vg::Paint::color(vg::Color::rgba(255, 255, 255, 25));
        paint.set_line_width(track_width);
        paint.set_line_cap(vg::LineCap::Round);
        canvas.stroke_path(&path, &paint);

        // Value arc
        if self.bipolar {
            let mid_angle = ARC_START + 0.5 * (ARC_END - ARC_START);
            let (from, to) = if value_angle < mid_angle {
                (value_angle, mid_angle)
            } else {
                (mid_angle, value_angle)
            };
            if (to - from).abs() > 0.01 {
                let mut path = vg::Path::new();
                path.arc(center_x, center_y, radius, from, to, vg::Solidity::Hole);
                let mut paint = vg::Paint::color(vg::Color::rgba(10, 132, 255, 220));
                paint.set_line_width(value_width);
                paint.set_line_cap(vg::LineCap::Round);
                canvas.stroke_path(&path, &paint);
            }
        } else if (value_angle - ARC_START).abs() > 0.01 {
            let mut path = vg::Path::new();
            path.arc(center_x, center_y, radius, ARC_START, value_angle, vg::Solidity::Hole);
            let mut paint = vg::Paint::color(vg::Color::rgba(10, 132, 255, 220));
            paint.set_line_width(value_width);
            paint.set_line_cap(vg::LineCap::Round);
            canvas.stroke_path(&path, &paint);
        }

        // Indicator dot
        let dot_x = center_x + radius * value_angle.cos();
        let dot_y = center_y + radius * value_angle.sin();
        let dot_radius = (size * 0.05).max(2.0);

        let mut path = vg::Path::new();
        path.circle(dot_x, dot_y, dot_radius);
        let paint = vg::Paint::color(vg::Color::rgb(255, 255, 255));
        canvas.fill_path(&path, &paint);
    }
}
