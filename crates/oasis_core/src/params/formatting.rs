use std::sync::Arc;

pub fn v2s_percentage() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| format!("{:.1}%", v * 100.0))
}

pub fn s2v_percentage() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches('%')
            .trim()
            .parse::<f32>()
            .ok()
            .map(|v| v / 100.0)
    })
}

pub fn v2s_width() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| format!("{:.0}%", v * 100.0))
}

pub fn s2v_width() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches('%')
            .trim()
            .parse::<f32>()
            .ok()
            .map(|v| v / 100.0)
    })
}
