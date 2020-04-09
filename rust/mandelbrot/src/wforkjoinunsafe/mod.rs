use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render_fork_join_unsafe};
use crate::time::{Clock, MyTimer};
use num::Complex;
use std::cell::UnsafeCell;
use std::mem;
use std::sync::Arc;
use std::thread;

pub struct Wrapper<T>(pub UnsafeCell<T>);
unsafe impl<T> Send for Wrapper<T> {}
unsafe impl<T> Sync for Wrapper<T> {}

pub fn time_fork_join_unsafe(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<f64, CustomError> {
    let arr_size = bounds.0 * bounds.1;
    let v = vec![0 as u8; arr_size];
    let mut v = mem::ManuallyDrop::new(v);
    let p: *mut u8 = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let pixels = Arc::new(Wrapper(UnsafeCell::new(p)));

    let number_of_threads = 8;
    let rows_per_band = bounds.1 / number_of_threads + 1;
    let chunk_size = rows_per_band * bounds.0;
    let mut threads = vec![];

    let mut start = MyTimer::new();
    let mut end = MyTimer::new();

    start.gettime(Clock::ClockMonotonicRaw)?;
    for (i, offset) in (0..arr_size).step_by(chunk_size).enumerate() {
        let pixels_ref = pixels.clone();
        let chunk_length = if arr_size - offset > chunk_size {
            chunk_size
        } else {
            arr_size - offset
        };
        let top = rows_per_band * i;
        let height = chunk_length / bounds.0;
        let band_bounds = (bounds.0, height);
        let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right =
            pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
        threads.push(thread::spawn(move || -> Result<(), CustomError> {
            render_fork_join_unsafe(
                pixels_ref,
                offset,
                band_bounds,
                band_upper_left,
                band_lower_right,
            )?;
            Ok(())
        }));
    }

    for thread in threads {
        match thread.join() {
            Ok(_) => {}
            Err(_) => return Err(CustomError::ThreadPanic),
        }
    }
    end.gettime(Clock::ClockMonotonicRaw)?;
    unsafe {
        let rebuilt = Vec::from_raw_parts(*pixels.0.get(), len, cap);
        //crate::mandel::write_image("mandel.png", &rebuilt, bounds)?;
    }

    drop(v);
    Ok(start.compute_time_millis(end))
}
