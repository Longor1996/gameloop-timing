//! Implementation of a tickloop according to <http://www.koonsolo.com/news/dewitters-gameloop/>

use super::*;

/// Implementation of a tickloop according to <http://www.koonsolo.com/news/dewitters-gameloop/>
pub struct TickLoopState {
    /// Start.
    start: Instant,
    
    /// The amount of ticks to attempt per second.
    ticks_per_second: u32,

    /// The normal duration between individual ticks.
    tick_duration: Duration,

    /// The normal duration between individual tocks.
    tock_duration: Duration,

    /// How many frames can be skipped when catching up with lost ticks.
    max_frameskip: i32,

    /// Counter of catch-up ticks per frame.
    loops: i32,

    /// The moment when the next tick happens.
    next_game_tick: Instant,

    /// The moment when the next tock happens.
    next_game_tock: Instant,

    /// The time when the last tick finished.
    last_tick_time: Instant,

    /// The time when the last tock occurred.
    last_tock_time: Instant,
    
    /// The x-per-second counter for ticks.
    tick_count: u32,

    /// Total number of ticks the gameloop went trough.
    total_ticks: u64,

    /// Total number of tocks the gameloop went trough.
    total_tocks: u64,
}

/// Event that is spawned when a tick occurs.
#[derive(Debug)]
pub struct TickLoopEvent {
    /// The targeted tick-rate.
    pub target_tickrate: u32,
    
    /// The current point in time.
    pub time: Instant,
    
    /// The intended tick [`Duration`].
    pub duration: Duration,
    
    /// The total number of ticks the gameloop went trough.
    pub ticks: u64,
}

/// Event that is spawned when a tock occurs (eg: once per second).
#[derive(Debug)]
pub struct TockLoopEvent {
    /// The targeted tick-rate.
    pub target_tickrate: u32,
    
    /// The current point in time.
    pub time: Instant,
    
    /// The intended tick [`Duration`].
    pub duration: Duration,
    
    /// The total number of ticks the gameloop went trough.
    pub ticks: u64,
    
    /// The total number of tocks the gameloop went trough.
    pub tocks: u64,
    
    /// The time of the last tock to occur.
    pub last_tock: Instant,
    
    /// The average tick-rate (TPS).
    pub average_tickrate: f64,
}

impl TickLoopState {
    /// Creates a new tickloop with the given target tick-rate.
    pub fn new(ticks_per_second: u32) -> Self {
        Self {
            start: Instant::now(),
            ticks_per_second,
            tick_duration: Duration::from_secs(1) / ticks_per_second,
            tock_duration: Duration::from_secs(1),
            max_frameskip: 1,
            loops: 0,
            next_game_tick: Instant::now(),
            next_game_tock: Instant::now(),
            last_tick_time: Instant::now(),
            last_tock_time: Instant::now(),
            tick_count: 0,
            total_ticks: 0,
            total_tocks: 0,
        }
    }
    
    /// Resets the tickloop.
    pub fn pre(&mut self) {
        self.loops = 0;
    }
    
    /// Attempt to do a tick (at current tickrate per second), returning `true` if one happened.
    pub fn tick<F: FnOnce(&mut TickLoopEvent)>(&mut self, current_time: Instant, function: F) -> bool {
        
        if (current_time > self.next_game_tick) && (self.loops < self.max_frameskip) {
            function(&mut TickLoopEvent {
                target_tickrate: self.ticks_per_second,
                time: current_time,
                duration: self.tick_duration,
                ticks: self.total_ticks,
            });
            
            self.last_tick_time = current_time;
            self.next_game_tick += self.tick_duration;
            self.loops += 1;
            self.tick_count += 1;
            self.total_ticks += 1;
            
            return true;
        }
        
        false
    }
    
    /// Attempt to do a tock (once per second), returning `true` if one happened.
    pub fn tock<F: FnOnce(&mut TockLoopEvent)>(&mut self, current_time: Instant, function: F) -> bool {
        
        let time_since_tock = current_time.duration_since(self.last_tock_time).as_secs_f64();
        
        if self.next_game_tock <= current_time {
            let hertz_avg = self.tick_count as f64 / time_since_tock;
            
            function(&mut TockLoopEvent {
                target_tickrate: self.ticks_per_second,
                time: current_time,
                duration: self.tick_duration,
                last_tock: self.last_tock_time,
                average_tickrate: hertz_avg,
                ticks: self.total_ticks,
                tocks: self.total_tocks,
            });
            
            self.tick_count = 0;
            self.total_tocks += 1;
            self.last_tock_time = current_time;
            self.next_game_tock = current_time + self.tock_duration;
            return true;
        }
        
        false
    }
    
    /// Returns the interpolation factor between the previous and next tick, for smooth rendering.
    pub fn interpolation(&self, current_time: Instant) -> f64 {
        let delta = current_time - self.next_game_tick;
        (delta + self.tick_duration).as_secs_f64() / self.tick_duration.as_secs_f64()
    }
    
    /// Returns the intended minimum duration of a single tick.
    pub fn get_minimum_tick_duration(&self) -> Duration {
        self.tick_duration
    }
    
    /// Returns when the tickloop was created.
    pub fn get_start(&self) -> Instant {
        self.start
    }
}
