/// SVG line-draw animation — anime.js `svg.createDrawable()` equivalent.
/// Animates `stroke-dashoffset` to create the effect of an SVG path being drawn.

use super::tween::{Easing, Tween};

/// State for an SVG draw animation
#[derive(Clone, Debug)]
pub struct SvgDraw {
    /// Total path length (from SVGPathElement.getTotalLength())
    path_length: f64,
    /// Tween controlling the dashoffset from path_length to 0
    tween: Tween,
}

impl SvgDraw {
    /// Create a new SVG draw animation.
    /// `path_length` should come from calling getTotalLength() on the SVG path element.
    pub fn new(path_length: f64, duration_ms: f64) -> Self {
        Self {
            path_length,
            tween: Tween::new(path_length, 0.0, duration_ms)
                .with_easing(Easing::EaseInOutCubic),
        }
    }

    pub fn with_delay(mut self, delay_ms: f64) -> Self {
        self.tween = self.tween.with_delay(delay_ms);
        self
    }

    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.tween = self.tween.with_easing(easing);
        self
    }

    /// Get the stroke-dasharray value (always the full path length)
    pub fn dash_array(&self) -> f64 {
        self.path_length
    }

    /// Get the stroke-dashoffset at the given elapsed time.
    /// Apply this to the SVG element's style.
    pub fn dash_offset_at(&self, elapsed_ms: f64) -> f64 {
        self.tween.value_at(elapsed_ms)
    }

    /// Is the draw animation complete?
    pub fn is_complete(&self, elapsed_ms: f64) -> bool {
        self.tween.is_complete(elapsed_ms)
    }

    pub fn total_duration_ms(&self) -> f64 {
        self.tween.total_duration_ms()
    }
}

/// Helper: Generate SVG path data for a simple circuit board pattern
/// Returns a string suitable for the `d` attribute of an SVG path
pub fn circuit_path(width: f64, height: f64) -> String {
    let w = width;
    let h = height;
    format!(
        "M 10,{midy} L {q1x},{midy} L {q1x},{q1y} L {midx},{q1y} \
         L {midx},{midy} L {q3x},{midy} L {q3x},{q3y} L {endx},{q3y} \
         M {midx},{midy} L {midx},{q3y} L {q1x},{q3y} \
         M {q3x},{midy} L {endx},{midy}",
        midy = h / 2.0,
        q1x = w * 0.2,
        q1y = h * 0.25,
        midx = w * 0.5,
        q3x = w * 0.75,
        q3y = h * 0.75,
        endx = w - 10.0,
    )
}
