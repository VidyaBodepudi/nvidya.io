/// Spring physics solver — anime.js `spring(mass, stiffness, damping, velocity)` equivalent.
/// Solves the damped harmonic oscillator differential equation:
///   acceleration = -stiffness * displacement - damping * velocity
///
/// This gives natural, organic motion that CSS transitions can never achieve.

/// Configuration for a spring animation
#[derive(Clone, Debug)]
pub struct SpringConfig {
    pub mass: f64,
    pub stiffness: f64,
    pub damping: f64,
    pub initial_velocity: f64,
    /// The spring is considered "at rest" when displacement and velocity
    /// are both below this threshold
    pub rest_threshold: f64,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            mass: 1.0,
            stiffness: 100.0,
            damping: 10.0,
            initial_velocity: 0.0,
            rest_threshold: 0.001,
        }
    }
}

impl SpringConfig {
    /// Preset: Gentle — slow, smooth settle
    pub fn gentle() -> Self {
        Self {
            mass: 1.0,
            stiffness: 50.0,
            damping: 14.0,
            initial_velocity: 0.0,
            rest_threshold: 0.001,
        }
    }

    /// Preset: Wobbly — more bounce
    pub fn wobbly() -> Self {
        Self {
            mass: 1.0,
            stiffness: 180.0,
            damping: 12.0,
            initial_velocity: 0.0,
            rest_threshold: 0.001,
        }
    }

    /// Preset: Stiff — snappy with minimal overshoot
    pub fn stiff() -> Self {
        Self {
            mass: 1.0,
            stiffness: 300.0,
            damping: 26.0,
            initial_velocity: 0.0,
            rest_threshold: 0.001,
        }
    }

    /// Preset: Bouncy — lots of overshoot
    pub fn bouncy() -> Self {
        Self {
            mass: 1.0,
            stiffness: 200.0,
            damping: 8.0,
            initial_velocity: 0.0,
            rest_threshold: 0.001,
        }
    }
}

/// A running spring simulation state
#[derive(Clone, Debug)]
pub struct SpringState {
    pub config: SpringConfig,
    pub from: f64,
    pub to: f64,
    displacement: f64,
    velocity: f64,
    at_rest: bool,
}

impl SpringState {
    pub fn new(from: f64, to: f64, config: SpringConfig) -> Self {
        Self {
            displacement: from - to,
            velocity: config.initial_velocity,
            at_rest: false,
            config,
            from,
            to,
        }
    }

    /// Advance the spring simulation by `dt` milliseconds.
    /// Uses semi-implicit Euler integration with a fixed sub-step for stability.
    pub fn step(&mut self, dt_ms: f64) {
        if self.at_rest {
            return;
        }

        let dt_s = dt_ms / 1000.0;
        // Use fixed sub-steps for stability (1ms each)
        let steps = (dt_s * 1000.0).ceil() as usize;
        let sub_dt = dt_s / steps as f64;

        for _ in 0..steps {
            let spring_force = -self.config.stiffness * self.displacement;
            let damping_force = -self.config.damping * self.velocity;
            let acceleration = (spring_force + damping_force) / self.config.mass;

            self.velocity += acceleration * sub_dt;
            self.displacement += self.velocity * sub_dt;
        }

        // Check if at rest
        if self.displacement.abs() < self.config.rest_threshold
            && self.velocity.abs() < self.config.rest_threshold
        {
            self.displacement = 0.0;
            self.velocity = 0.0;
            self.at_rest = true;
        }
    }

    /// Get the current value of the spring
    pub fn value(&self) -> f64 {
        self.to + self.displacement
    }

    /// Is the spring at rest?
    pub fn is_at_rest(&self) -> bool {
        self.at_rest
    }

    /// Reset the spring to target a new value
    pub fn retarget(&mut self, new_to: f64) {
        self.displacement = self.value() - new_to;
        self.to = new_to;
        self.at_rest = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spring_settles() {
        let mut spring = SpringState::new(0.0, 100.0, SpringConfig::default());
        // Run for 5 seconds of simulation
        for _ in 0..300 {
            spring.step(16.67); // ~60fps
        }
        assert!(spring.is_at_rest());
        assert!((spring.value() - 100.0).abs() < 0.01);
    }

    #[test]
    fn spring_overshoots_when_bouncy() {
        let mut spring = SpringState::new(0.0, 100.0, SpringConfig::bouncy());
        let mut max_val = 0.0_f64;
        for _ in 0..300 {
            spring.step(16.67);
            max_val = max_val.max(spring.value());
        }
        // Bouncy spring should overshoot past 100
        assert!(max_val > 100.0);
    }
}
