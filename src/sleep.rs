//! Utility for when a mainloop must sleep.

use super::{looptimer::LoopTimer, *};

/// Yields the current threads time-slice...
///
/// ...until we're close enough to all the given loops next intended iteration,
/// at which point we switch to a busyloop.
pub fn sleep_if_needed(timers: &[&LoopTimer], min: Option<Duration>) {
    let min = min.unwrap_or_else(|| Duration::from_nanos(1000));
    
    loop {
        let now = Instant::now();

        match timers.iter().fold(Decider::YieldNow, |mut d, t| {
            let sleep = now - t.end;
            let target = t.target - Duration::from_millis(1);

            // If we are out of time, break.
            if sleep >= target {
                d = Decider::Break
            }

            // If we haven't slept long enough, we yield.
            if sleep < target {
                d = d.fold(Decider::YieldNow)
            }

            // If we have slept for a very short time, busyloop.
            if sleep > min {
                d = d.fold(Decider::BusyLoop)
            }

            d
        }) {
            Decider::Break => return,
            Decider::BusyLoop => continue,
            Decider::YieldNow => std::thread::yield_now(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Decider {
    Break,
    BusyLoop,
    YieldNow,
}

impl Decider {
    fn fold(&self, other: Self) -> Self {
        match self {
            Decider::Break => Decider::Break,
            Decider::BusyLoop => {
                if other == Self::Break {
                    other
                } else {
                    *self
                }
            }
            Decider::YieldNow => {
                if other != Self::YieldNow {
                    other
                } else {
                    *self
                }
            }
        }
    }
}
