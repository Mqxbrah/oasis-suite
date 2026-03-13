use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

#[derive(Clone, Copy, PartialEq)]
pub enum ArrowDirection {
    Left,
    Right,
}

/// A clickable arrow button that draws a filled triangle.
/// Emits a configurable event on click.
pub struct ArrowButton<E: 'static + Clone + Send> {
    direction: ArrowDirection,
    on_click: E,
}

impl<E: 'static + Clone + Send> ArrowButton<E> {
    pub fn new(cx: &mut Context, direction: ArrowDirection, on_click: E) -> Handle<'_, Self>
    where
        E: std::fmt::Debug,
    {
        Self { direction, on_click }.build(cx, |_| {})
    }
}

impl<E: 'static + Clone + Send + std::fmt::Debug> View for ArrowButton<E> {
    fn element(&self) -> Option<&'static str> {
        Some("arrow-btn")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| {
            if let WindowEvent::MouseDown(MouseButton::Left) = *window_event {
                cx.emit(self.on_click.clone());
                meta.consume();
            }
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        if bounds.w == 0.0 || bounds.h == 0.0 {
            return;
        }

        // Background
        let bg_color = cx.background_color();
        let mut bg: vg::Color = bg_color.into();
        bg.set_alphaf(bg.a * cx.opacity());

        if bg.a > 0.01 {
            let mut path = vg::Path::new();
            path.rounded_rect(bounds.x, bounds.y, bounds.w, bounds.h, 4.0);
            canvas.fill_path(&path, &vg::Paint::color(bg));
        }

        // Triangle
        let center_x = bounds.x + bounds.w * 0.5;
        let center_y = bounds.y + bounds.h * 0.5;
        let tri_w = bounds.w * 0.28;
        let tri_h = bounds.h * 0.28;

        let mut path = vg::Path::new();
        match self.direction {
            ArrowDirection::Left => {
                path.move_to(center_x - tri_w, center_y);
                path.line_to(center_x + tri_w, center_y - tri_h);
                path.line_to(center_x + tri_w, center_y + tri_h);
            }
            ArrowDirection::Right => {
                path.move_to(center_x + tri_w, center_y);
                path.line_to(center_x - tri_w, center_y - tri_h);
                path.line_to(center_x - tri_w, center_y + tri_h);
            }
        }
        path.close();

        let fg: vg::Color = cx.font_color().into();
        let color = if fg.a < 0.01 {
            vg::Color::rgb(152, 152, 157)
        } else {
            fg
        };
        canvas.fill_path(&path, &vg::Paint::color(color));
    }
}
