mod scroll;

pub use scroll::ScrollAnimation;

use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Easing {
    Linear,
    EaseOut,
    EaseInOut,
}

impl Easing {
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Easing::Linear => t,
            Easing::EaseOut => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOut => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
        }
    }
}

pub struct AnimationConfig {
    pub duration: Duration,
    pub easing: Easing,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(150),
            easing: Easing::EaseOut,
        }
    }
}
