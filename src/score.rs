use crate::consts::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,
    score: usize,
}

impl ScoreResource {
    /// Increases number of corrects and adds to score
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        // Get a value from 0 to 1 according to how close the press was
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        // Give at least 10 points and max 100 points
        let points = (score_multiplier * 100.0).min(100.0).max(10.0) as usize;
        self.score += points;

        points
    }

    /// Increases the number of failures.
    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }

    /// Get the score
    pub fn score(&self) -> usize {
        self.score
    }

    /// Get the number of corrects
    pub fn corrects(&self) -> usize {
        self.corrects
    }

    /// Get the number of fails
    pub fn fails(&self) -> usize {
        self.fails
    }
}
