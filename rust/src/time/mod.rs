use crate::customerror::CustomError;
use libc::c_int;
use libc::timespec;

///The clock_gettime function of c inbound in Rust
#[link(name = "c")]
extern "C" {
    pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
}

pub enum Clock {
    ClockMonotonicRaw,
}

impl Clock {
    /// Returns the the Enum value
    pub fn value(&self) -> i32 {
        match *self {
            Clock::ClockMonotonicRaw => 4,
        }
    }
}

///A Wrapper around the timespec struct from c.
#[derive(Clone)]
pub struct MyTimestamp {
    pub ts: timespec,
}

impl Default for MyTimestamp {
    fn default() -> Self {
        Self::new()
    }
}

impl MyTimestamp {
    ///Returns an initialized Struct MyTimestamp
    pub fn new() -> Self {
        MyTimestamp {
            ts: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
        }
    }

    ///Computes the passed time between two timestamps of type MyTimestamp in ms.
    pub fn compute_time_millis(&self, end: Self) -> f64 {
        (end.ts.tv_sec - self.ts.tv_sec) as f64 * 1000.0
            + (end.ts.tv_nsec - self.ts.tv_nsec) as f64 / 1_000_000.0
    }

    ///Call of the c syscall clock_gettime to obtain a timestamp.
    pub fn gettime(&mut self, clock: Clock) -> Result<(), CustomError> {
        if unsafe { clock_gettime(clock.value(), &mut self.ts) } == -1 {
            Err(CustomError::TimerError)
        } else {
            Ok(())
        }
    }
}
