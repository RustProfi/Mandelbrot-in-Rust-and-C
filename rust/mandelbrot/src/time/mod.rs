extern crate libc;
use crate::customerror::CustomError;
use libc::c_int;
use libc::timespec;

#[link(name = "c")]
extern "C" {
    pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
}

pub enum Clock {
    ClockMonotonicRaw,
}

impl Clock {
    pub fn value(&self) -> i32 {
        match *self {
            Clock::ClockMonotonicRaw => 4,
        }
    }
}

pub struct MyTimer {
    ts: timespec,
}

impl Default for MyTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl MyTimer {
    pub fn new() -> Self {
        MyTimer {
            ts: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
        }
    }

    pub fn compute_time_millis(&self, end: Self) -> f64 {
        (end.ts.tv_sec - self.ts.tv_sec) as f64 * 1000.0
            + (1_000_000_000 - self.ts.tv_nsec + end.ts.tv_nsec) as f64 / 1_000_000.0
    }

    pub fn gettime(&mut self, clock: Clock) -> Result<(), CustomError> {
        if unsafe { clock_gettime(clock.value(), &mut self.ts) } == -1 {
            Err(CustomError::TimerError)
        } else {
            Ok(())
        }
    }
}
