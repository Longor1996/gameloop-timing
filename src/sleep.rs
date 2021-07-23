//! Utility for when a mainloop must sleep.

use super::{looptimer::LoopTimer, *};

/// Yields the current threads time-slice...
/// 
/// ...until we're close enough to all the given loops next intended iteration,
/// at which point we switch to a busyloop.
pub fn sleep_if_needed(
    timers: &[&LoopTimer],
    min: Option<Duration>
) {
    let min = min.unwrap_or_else(|| Duration::from_nanos(100000));
    loop {
        let now = Instant::now();
        
        if timers.iter().all(|t| {
            let sleep = now - t.end;
            sleep < t.target && sleep > min
        }) {
            std::thread::yield_now();
        } else {
            return;
        }
    }
}
