use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

#[derive(Clone, Copy, PartialEq)]
pub enum ArrowDirection {
    Left,
    Right,
}

pub struct Arrow {
    direction: ArrowDirection,
}

impl Arrow {
    pub fn new(cx: &mut Context, direction: ArrowDirection) -> Handle<'_, Self> {
        Self { direction }.build(cx, |_| {})
    }
}

impl View for Arrow {
    fn element(&self) -> Option<&'static str> {
        Some("arrow-icon")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        let w = bounds.w;
        let h = bounds.h;
        let cx_pos = bounds.x + w * 0.5;
        let cy_pos = bounds.y + h * 0.5;

        let tri_w = w * 0.35;
        let tri_h = h * 0.4;

        let mut path = vg::Path::new();
        match self.direction {
            ArrowDirection::Left => {
                path.move_to(cx_pos - tri_w * 0.5, cy_pos);
                path.line_to(cx_pos + tri_w * 0.5, cy_pos - tri_h);
                path.line_to(cx_pos + tri_w * 0.5, cy_pos + tri_h);
            }
            ArrowDirection::Right => {
                path.move_to(cx_pos + tri_w * 0.5, cy_pos);
                path.line_to(cx_pos - tri_w * 0.5, cy_pos - tri_h);
                path.line_to(cx_pos - tri_w * 0.5, cy_pos + tri_h);
            }
        }
        path.close();

        let opacity = cx.opacity();
        let mut color: vg::Color = cx.font_color().into();
        color.set_alphaf(color.a * opacity);
        let paint = vg::Paint::color(color);
        canvas.fill_path(&path, &paint);
    }
}
