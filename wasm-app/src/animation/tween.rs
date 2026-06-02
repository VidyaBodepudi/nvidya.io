/// Easing functions inspired by anime.js — all take `t` in [0.0, 1.0] and return eased value.
/// These are pure math functions with zero allocation.

use std::f64::consts::PI;

/// Linear (no easing)
pub fn linear(t: f64) -> f64 {
    t
}

/// Ease in quad — slow start
pub fn ease_in_quad(t: f64) -> f64 {
    t * t
}

/// Ease out quad — slow end
pub fn ease_out_quad(t: f64) -> f64 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Ease in-out quad
pub fn ease_in_out_quad(t: f64) -> f64 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

/// Ease out cubic — smooth deceleration
pub fn ease_out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

/// Ease in-out cubic
pub fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Ease out elastic — bouncy overshoot like a spring snapping
pub fn ease_out_elastic(t: f64) -> f64 {
    if t == 0.0 || t == 1.0 {
        return t;
    }
    let c4 = (2.0 * PI) / 3.0;
    2.0_f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
}

/// Ease out back — slight overshoot then settle
pub fn ease_out_back(t: f64) -> f64 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

/// Ease out bounce — multiple bounces
pub fn ease_out_bounce(t: f64) -> f64 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

/// Easing function type — allows selecting easing at runtime
#[derive(Clone, Copy, Debug)]
pub enum Easing {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseOutCubic,
    EaseInOutCubic,
    EaseOutElastic,
    EaseOutBack,
    EaseOutBounce,
}

impl Easing {
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Easing::Linear => linear(t),
            Easing::EaseInQuad => ease_in_quad(t),
            Easing::EaseOutQuad => ease_out_quad(t),
            Easing::EaseInOutQuad => ease_in_out_quad(t),
            Easing::EaseOutCubic => ease_out_cubic(t),
            Easing::EaseInOutCubic => ease_in_out_cubic(t),
            Easing::EaseOutElastic => ease_out_elastic(t),
            Easing::EaseOutBack => ease_out_back(t),
            Easing::EaseOutBounce => ease_out_bounce(t),
        }
    }
}

/// A tween that interpolates a single f64 value from `start` to `end` over `duration_ms`.
#[derive(Clone, Debug)]
pub struct Tween {
    pub start_val: f64,
    pub end_val: f64,
    pub duration_ms: f64,
    pub delay_ms: f64,
    pub easing: Easing,
}

impl Tween {
    pub fn new(start: f64, end: f64, duration_ms: f64) -> Self {
        Self {
            start_val: start,
            end_val: end,
            duration_ms,
            delay_ms: 0.0,
            easing: Easing::EaseOutCubic,
        }
    }

    pub fn with_delay(mut self, delay_ms: f64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Get the interpolated value at a given elapsed time (ms).
    /// Returns None if the tween hasn't started yet (still in delay).
    /// Returns the end value if the tween is complete.
    pub fn value_at(&self, elapsed_ms: f64) -> f64 {
        let local_time = elapsed_ms - self.delay_ms;
        if local_time <= 0.0 {
            return self.start_val;
        }
        if local_time >= self.duration_ms {
            return self.end_val;
        }
        let t = local_time / self.duration_ms;
        let eased = self.easing.apply(t);
        self.start_val + (self.end_val - self.start_val) * eased
    }

    /// Returns true if the tween is complete at the given elapsed time
    pub fn is_complete(&self, elapsed_ms: f64) -> bool {
        elapsed_ms >= self.delay_ms + self.duration_ms
    }

    /// Total duration including delay
    pub fn total_duration_ms(&self) -> f64 {
        self.delay_ms + self.duration_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tween_basic() {
        let tw = Tween::new(0.0, 100.0, 1000.0);
        assert_eq!(tw.value_at(0.0), 0.0);
        assert_eq!(tw.value_at(1000.0), 100.0);
        // Mid-point should be between 0 and 100
        let mid = tw.value_at(500.0);
        assert!(mid > 0.0 && mid < 100.0);
    }

    #[test]
    fn tween_with_delay() {
        let tw = Tween::new(0.0, 100.0, 1000.0).with_delay(500.0);
        assert_eq!(tw.value_at(0.0), 0.0);
        assert_eq!(tw.value_at(500.0), 0.0);
        assert_eq!(tw.value_at(1500.0), 100.0);
    }
}
