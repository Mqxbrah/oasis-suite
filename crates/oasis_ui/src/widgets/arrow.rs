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
        let center_x = bounds.x + w * 0.5;
        let center_y = bounds.y + h * 0.5;

        let tri_w = w * 0.3;
        let tri_h = h * 0.3;

        let mut path = vg::Path::new();
        match self.direction {
            ArrowDirection::Left => {
                path.move_to(center_x - tri_w * 0.5, center_y);
                path.line_to(center_x + tri_w * 0.5, center_y - tri_h);
                path.line_to(center_x + tri_w * 0.5, center_y + tri_h);
            }
            ArrowDirection::Right => {
                path.move_to(center_x + tri_w * 0.5, center_y);
                path.line_to(center_x - tri_w * 0.5, center_y - tri_h);
                path.line_to(center_x - tri_w * 0.5, center_y + tri_h);
            }
        }
        path.close();

        let color: vg::Color = cx.font_color().into();
        // Fall back to a visible gray if font_color comes back as fully transparent
        let paint = if color.a < 0.01 {
            vg::Paint::color(vg::Color::rgb(152, 152, 157))
        } else {
            vg::Paint::color(color)
        };
        canvas.fill_path(&path, &paint);
    }
}
