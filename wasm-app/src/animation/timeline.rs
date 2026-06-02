/// Timeline sequencer — anime.js `createTimeline()` equivalent.
/// Orchestrates multiple animation phases with precise start times.
/// A single `request_animation_frame` loop drives the entire timeline.

use super::tween::{Easing, Tween};

/// A single phase in the timeline with a label and multiple tweens
#[derive(Clone, Debug)]
pub struct TimelinePhase {
    pub label: String,
    pub start_ms: f64,
    pub tweens: Vec<NamedTween>,
}

/// A tween with a name (e.g. "opacity", "translateY") for identification
#[derive(Clone, Debug)]
pub struct NamedTween {
    pub name: String,
    pub tween: Tween,
}

/// The timeline itself
#[derive(Clone, Debug)]
pub struct Timeline {
    pub phases: Vec<TimelinePhase>,
    total_duration_ms: f64,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            phases: Vec::new(),
            total_duration_ms: 0.0,
        }
    }

    /// Add a phase at an absolute start time
    pub fn add_phase(mut self, label: &str, start_ms: f64, tweens: Vec<NamedTween>) -> Self {
        let phase_end = tweens
            .iter()
            .map(|nt| start_ms + nt.tween.total_duration_ms())
            .fold(0.0_f64, f64::max);

        if phase_end > self.total_duration_ms {
            self.total_duration_ms = phase_end;
        }

        self.phases.push(TimelinePhase {
            label: label.to_string(),
            start_ms,
            tweens,
        });
        self
    }

    /// Add a phase that starts relative to the end of the last phase
    pub fn then(self, label: &str, offset_ms: f64, tweens: Vec<NamedTween>) -> Self {
        let start = self.total_duration_ms + offset_ms;
        self.add_phase(label, start, tweens)
    }

    /// Get total duration of the timeline
    pub fn total_duration(&self) -> f64 {
        self.total_duration_ms
    }

    /// Query all tween values at a given elapsed time.
    /// Returns a Vec of (phase_label, tween_name, value) tuples for active/completed tweens.
    pub fn values_at(&self, elapsed_ms: f64) -> Vec<(&str, &str, f64)> {
        let mut results = Vec::new();

        for phase in &self.phases {
            let phase_elapsed = elapsed_ms - phase.start_ms;
            if phase_elapsed < 0.0 {
                // Phase hasn't started yet — emit start values
                for nt in &phase.tweens {
                    results.push((
                        phase.label.as_str(),
                        nt.name.as_str(),
                        nt.tween.start_val,
                    ));
                }
            } else {
                for nt in &phase.tweens {
                    results.push((
                        phase.label.as_str(),
                        nt.name.as_str(),
                        nt.tween.value_at(phase_elapsed),
                    ));
                }
            }
        }

        results
    }

    /// Is the entire timeline complete?
    pub fn is_complete(&self, elapsed_ms: f64) -> bool {
        elapsed_ms >= self.total_duration_ms
    }
}

/// Helper to create a named tween quickly
pub fn named(name: &str, start: f64, end: f64, duration_ms: f64, easing: Easing) -> NamedTween {
    NamedTween {
        name: name.to_string(),
        tween: Tween::new(start, end, duration_ms).with_easing(easing),
    }
}

/// Helper to create a named tween with delay
pub fn named_delayed(
    name: &str,
    start: f64,
    end: f64,
    duration_ms: f64,
    delay_ms: f64,
    easing: Easing,
) -> NamedTween {
    NamedTween {
        name: name.to_string(),
        tween: Tween::new(start, end, duration_ms)
            .with_delay(delay_ms)
            .with_easing(easing),
    }
}
