use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

/// A self-directed overlay panel that draws its own opaque background.
/// Use this instead of vizia's Popup when you need a guaranteed visible background.
pub struct DropdownOverlay;

impl DropdownOverlay {
    pub fn new<L, F>(cx: &mut Context, visible: L, content: F) -> Handle<'_, Self>
    where
        L: Lens<Target = bool> + Copy,
        F: 'static + Fn(&mut Context),
    {
        Self.build(cx, move |cx| {
            Binding::new(cx, visible, move |cx, vis| {
                if vis.get(cx) {
                    (content)(cx);
                }
            });
        })
        .toggle_class("dropdown-visible", visible)
    }
}

impl View for DropdownOverlay {
    fn element(&self) -> Option<&'static str> {
        Some("dropdown-overlay")
    }

    fn event(&mut self, _cx: &mut EventContext, _event: &mut Event) {
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        if bounds.w < 1.0 || bounds.h < 1.0 {
            return;
        }

        let opacity = cx.opacity();
        if opacity < 0.01 {
            return;
        }

        // Shadow
        let shadow_paint = vg::Paint::box_gradient(
            bounds.x, bounds.y + 2.0,
            bounds.w, bounds.h,
            6.0, 12.0,
            vg::Color::rgba(0, 0, 0, (180.0 * opacity) as u8),
            vg::Color::rgba(0, 0, 0, 0),
        );
        let mut shadow_path = vg::Path::new();
        shadow_path.rounded_rect(
            bounds.x - 4.0, bounds.y - 2.0,
            bounds.w + 8.0, bounds.h + 10.0,
            8.0,
        );
        canvas.fill_path(&shadow_path, &shadow_paint);

        // Background fill
        let mut bg_path = vg::Path::new();
        bg_path.rounded_rect(bounds.x, bounds.y, bounds.w, bounds.h, 6.0);
        let bg_paint = vg::Paint::color(vg::Color::rgba(
            37, 37, 40, (255.0 * opacity) as u8,
        ));
        canvas.fill_path(&bg_path, &bg_paint);

        // Border
        let mut border_paint = vg::Paint::color(vg::Color::rgba(
            74, 74, 76, (255.0 * opacity) as u8,
        ));
        border_paint.set_line_width(1.0);
        canvas.stroke_path(&bg_path, &border_paint);
    }
}
