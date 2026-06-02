/// Scramble text effect — anime.js `scrambleText` equivalent.
/// Progressively reveals target text by replacing characters with random glyphs,
/// then resolving left-to-right.

use rand::prelude::*;

/// Characters used for the scramble effect — katakana, hex, and symbols
/// matching the hacker/matrix theme
const GLITCH_CHARS: &str = "ｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾘﾙﾚﾛﾜﾝ0123456789ABCDEF@#$%&*";

/// State for a running scramble text animation
pub struct ScrambleText {
    target: Vec<char>,
    glitch_chars: Vec<char>,
    rng: SmallRng,
    /// Total duration in ms
    duration_ms: f64,
    /// How many frames of scramble before each character resolves
    /// (higher = more scramble)
    scramble_intensity: usize,
}

impl ScrambleText {
    pub fn new(target: &str, duration_ms: f64) -> Self {
        Self {
            target: target.chars().collect(),
            glitch_chars: GLITCH_CHARS.chars().collect(),
            rng: SmallRng::from_entropy(),
            duration_ms,
            scramble_intensity: 3,
        }
    }

    pub fn with_intensity(mut self, intensity: usize) -> Self {
        self.scramble_intensity = intensity;
        self
    }

    /// Get the display string at a given elapsed time.
    /// Characters resolve left-to-right over the duration.
    pub fn text_at(&mut self, elapsed_ms: f64) -> String {
        let len = self.target.len();

        if elapsed_ms <= 0.0 {
            return (0..len)
                .map(|_| self.random_char())
                .collect();
        }

        if elapsed_ms >= self.duration_ms {
            return self.target.iter().collect();
        }

        let progress = elapsed_ms / self.duration_ms;
        // Number of characters that are fully resolved
        let resolved_count = (progress * len as f64).floor() as usize;
        // Progress within the current character slot
        let char_progress = (progress * len as f64).fract();

        // Clone target to avoid borrow conflict with self.random_char()
        let target_clone: Vec<char> = self.target.clone();
        let mut result = String::with_capacity(len * 3); // UTF-8 chars can be multi-byte

        for (i, &target_char) in target_clone.iter().enumerate() {
            if i < resolved_count {
                // Already resolved — show real character
                result.push(target_char);
            } else if i == resolved_count {
                // Currently resolving — scramble but occasionally flash the real char
                if char_progress > 0.7 {
                    result.push(target_char);
                } else {
                    result.push(self.random_char());
                }
            } else {
                // Not yet reached — show random character
                if target_char == ' ' {
                    result.push(' ');
                } else {
                    result.push(self.random_char());
                }
            }
        }

        result
    }

    fn random_char(&mut self) -> char {
        let idx = self.rng.gen_range(0..self.glitch_chars.len());
        self.glitch_chars[idx]
    }

    pub fn is_complete(&self, elapsed_ms: f64) -> bool {
        elapsed_ms >= self.duration_ms
    }

    pub fn duration(&self) -> f64 {
        self.duration_ms
    }
}
