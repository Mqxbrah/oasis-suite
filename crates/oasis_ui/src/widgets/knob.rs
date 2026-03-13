use nih_plug::prelude::Param;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
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
}

enum KnobEvent {
    DragStart(f32, f32),
    DragMove(f32, bool),
    DragEnd,
    ResetToDefault,
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
        }
        .build(cx, |_| {})
    }
}

impl View for ParamKnob {
    fn element(&self) -> Option<&'static str> {
        Some("param-knob")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match *window_event {
            WindowEvent::MouseDown(MouseButton::Left) => {
                if cx.modifiers().alt() {
                    cx.emit(KnobEvent::ResetToDefault);
                } else {
                    let norm = unsafe { self.param_ptr.unmodulated_normalized_value() };
                    cx.emit(KnobEvent::DragStart(cx.mouse().cursory, norm));
                }
                meta.consume();
            }
            WindowEvent::MouseDoubleClick(MouseButton::Left) => {
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
            }
            KnobEvent::DragEnd => {
                cx.release();
                cx.set_active(false);
                self.is_dragging = false;
                cx.emit(RawParamEvent::EndSetParameter(self.param_ptr));
            }
            KnobEvent::ResetToDefault => {
                let default = unsafe { self.param_ptr.default_normalized_value() };
                cx.emit(RawParamEvent::BeginSetParameter(self.param_ptr));
                cx.emit(RawParamEvent::SetParameterNormalized(self.param_ptr, default));
                cx.emit(RawParamEvent::EndSetParameter(self.param_ptr));
            }
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
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

        // Track arc (dark background)
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

        // Indicator dot at current position
        let dot_x = center_x + radius * value_angle.cos();
        let dot_y = center_y + radius * value_angle.sin();
        let dot_radius = (size * 0.05).max(2.0);

        let mut path = vg::Path::new();
        path.circle(dot_x, dot_y, dot_radius);
        let paint = vg::Paint::color(vg::Color::rgb(255, 255, 255));
        canvas.fill_path(&path, &paint);
    }
}
