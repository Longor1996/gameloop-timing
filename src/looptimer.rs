//! A basic timer struct for looping constructs.

use super::*;

/// Wraps the variables necessary to keep track of loop timings.
pub struct LoopTimer {
    /// The last start of the loop.
    pub start: Instant,
    
    /// The last end of the loop.
    pub end: Instant,
    
    /// The last time/duration of the loop.
    pub time: Duration,
    
    /// The intended time/duration of the loop.
    pub target: Duration,
    
    /// How many iterations have been done.
    pub count: u32,
}

impl LoopTimer {
    /// Creates a new looptimer with the given target loop duration.
    pub fn new(target: Duration) -> Self {
        let now = Instant::now();
        Self {
            count: 0,
            start: now,
            end: now + target,
            time: target,
            target
        }
    }
    
    /// Creates a new looptimer with the given target loop duration and a starting instant.
    pub fn new_from(now: Instant, target: Duration) -> Self {
        Self {
            count: 0,
            start: now,
            end: now + target,
            time: target,
            target
        }
    }
    
    /// Resets the loop-counter.
    pub fn reset_count(&mut self) {
        self.count = 0;
    }
    
    /// Begins the loop measurement.
    pub fn start(&mut self) {
        self.start = Instant::now();
    }
    
    /// Finishes the loop measurement.
    pub fn end(&mut self) {
        self.end = Instant::now();
        self.time = self.end.saturating_duration_since(self.end);
        self.count += 1;
    }
    
    /// Returns the length of the last loop iteration, in nanoseconds.
    pub fn length(&self) -> u128 {
        self.time.as_nanos()
    }
}
