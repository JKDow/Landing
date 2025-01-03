use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarSystemOptions {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_star_count")]
    pub star_count: u32,
    #[serde(default = "default_clear_color")]
    pub clear_color: String,
    #[serde(default = "default_fade_speed")]
    pub fade_speed: f64,
    #[serde(default = "default_use_advanced")]
    pub use_advanced: bool,
    #[serde(default = "default_star_size")]
    pub star_size: f32,
}

impl Default for StarSystemOptions {
    fn default() -> Self {
        StarSystemOptions {
            log_level: default_log_level(),
            star_count: default_star_count(),
            clear_color: default_clear_color(),
            fade_speed: default_fade_speed(),
            use_advanced: default_use_advanced(),
            star_size: default_star_size(),
        }
    }
}

fn default_log_level() -> String {
    String::from("warn")
}

fn default_star_count() -> u32 {
    1000
}

fn default_clear_color() -> String {
    String::from("#000000")
}

fn default_fade_speed() -> f64 {
    0.001
}

fn default_use_advanced() -> bool {
    true
}

fn default_star_size() -> f32 {
    1.0
}
