/// Stagger delay calculator — anime.js `stagger()` equivalent.
/// Computes per-element delay values for a set of targets to create
/// cascade/wave/ripple effects.

/// Direction of the stagger
#[derive(Clone, Copy, Debug)]
pub enum StaggerDirection {
    /// Left to right (default)
    Normal,
    /// Right to left
    Reverse,
    /// Center outward
    Center,
}

/// Stagger configuration
#[derive(Clone, Debug)]
pub struct StaggerConfig {
    /// Base delay between each element (ms)
    pub interval_ms: f64,
    /// Starting delay offset (ms)
    pub start_ms: f64,
    /// Direction of the stagger
    pub direction: StaggerDirection,
}

impl Default for StaggerConfig {
    fn default() -> Self {
        Self {
            interval_ms: 100.0,
            start_ms: 0.0,
            direction: StaggerDirection::Normal,
        }
    }
}

impl StaggerConfig {
    pub fn new(interval_ms: f64) -> Self {
        Self {
            interval_ms,
            ..Default::default()
        }
    }

    pub fn with_start(mut self, start_ms: f64) -> Self {
        self.start_ms = start_ms;
        self
    }

    pub fn with_direction(mut self, direction: StaggerDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Calculate delays for `count` elements.
    /// Returns a Vec of delay values in milliseconds.
    pub fn delays(&self, count: usize) -> Vec<f64> {
        if count == 0 {
            return Vec::new();
        }

        let mut delays = Vec::with_capacity(count);

        match self.direction {
            StaggerDirection::Normal => {
                for i in 0..count {
                    delays.push(self.start_ms + i as f64 * self.interval_ms);
                }
            }
            StaggerDirection::Reverse => {
                for i in 0..count {
                    delays.push(self.start_ms + (count - 1 - i) as f64 * self.interval_ms);
                }
            }
            StaggerDirection::Center => {
                let center = (count as f64 - 1.0) / 2.0;
                for i in 0..count {
                    let distance = (i as f64 - center).abs();
                    delays.push(self.start_ms + distance * self.interval_ms);
                }
            }
        }

        delays
    }
}

/// 2D grid stagger — for grid layouts. Creates a radial wave from a center point.
pub fn grid_stagger(
    cols: usize,
    rows: usize,
    interval_ms: f64,
    center_x: f64,
    center_y: f64,
) -> Vec<f64> {
    let mut delays = Vec::with_capacity(cols * rows);

    for row in 0..rows {
        for col in 0..cols {
            let dx = col as f64 - center_x;
            let dy = row as f64 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            delays.push(distance * interval_ms);
        }
    }

    delays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stagger_normal() {
        let config = StaggerConfig::new(100.0);
        let delays = config.delays(3);
        assert_eq!(delays, vec![0.0, 100.0, 200.0]);
    }

    #[test]
    fn stagger_center() {
        let config = StaggerConfig::new(100.0).with_direction(StaggerDirection::Center);
        let delays = config.delays(5);
        // Center element should have 0 delay, outer elements have max delay
        assert_eq!(delays[2], 0.0);
        assert_eq!(delays[0], 200.0);
        assert_eq!(delays[4], 200.0);
    }
}
