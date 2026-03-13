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

pub fn v2s_db() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| format!("{:.1} dB", v))
}

pub fn s2v_db() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches(" dB")
            .trim_end_matches("dB")
            .trim()
            .parse::<f32>()
            .ok()
    })
}

pub fn v2s_ms() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| format!("{:.1} ms", v))
}

pub fn s2v_ms() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches(" ms")
            .trim_end_matches("ms")
            .trim()
            .parse::<f32>()
            .ok()
    })
}

pub fn v2s_hz() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| {
        if v >= 1000.0 {
            format!("{:.2} kHz", v / 1000.0)
        } else {
            format!("{:.1} Hz", v)
        }
    })
}

pub fn s2v_hz() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        let s = s.trim();
        if let Some(khz) = s.strip_suffix("kHz").or_else(|| s.strip_suffix(" kHz")) {
            khz.trim().parse::<f32>().ok().map(|v| v * 1000.0)
        } else {
            s.trim_end_matches(" Hz")
                .trim_end_matches("Hz")
                .trim()
                .parse::<f32>()
                .ok()
        }
    })
}

pub fn v2s_ratio() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| format!("{:.1}:1", v))
}

pub fn s2v_ratio() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches(":1")
            .trim()
            .parse::<f32>()
            .ok()
    })
}

pub fn v2s_semitones() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| {
        if v > 0.0 {
            format!("+{:.0} st", v)
        } else {
            format!("{:.0} st", v)
        }
    })
}

pub fn s2v_semitones() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches(" st")
            .trim_end_matches("st")
            .trim()
            .parse::<f32>()
            .ok()
    })
}

pub fn v2s_cents() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| {
        if v > 0.0 {
            format!("+{:.0} ct", v)
        } else {
            format!("{:.0} ct", v)
        }
    })
}

pub fn s2v_cents() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches(" ct")
            .trim_end_matches("ct")
            .trim()
            .parse::<f32>()
            .ok()
    })
}

pub fn v2s_bipolar_percent() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(|v| {
        let pct = v * 100.0;
        if pct > 0.0 {
            format!("+{:.0}%", pct)
        } else {
            format!("{:.0}%", pct)
        }
    })
}

pub fn s2v_bipolar_percent() -> Arc<dyn Fn(&str) -> Option<f32> + Send + Sync> {
    Arc::new(|s| {
        s.trim()
            .trim_end_matches('%')
            .trim()
            .parse::<f32>()
            .ok()
            .map(|v| v / 100.0)
    })
}
