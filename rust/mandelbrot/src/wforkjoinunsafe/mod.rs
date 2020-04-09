use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render_fork_join_unsafe};
use crate::time::{Clock, MyTimestamp};
use num::Complex;
use std::cell::UnsafeCell;
use std::mem;
use std::sync::Arc;
use std::thread;

///A wrapper around UnsafeCell<T> to make UnsafeCell Send and Sync
pub struct Wrapper<T>(pub UnsafeCell<T>);
unsafe impl<T> Send for Wrapper<T> {}
unsafe impl<T> Sync for Wrapper<T> {}

///Measure in ms how long it takes to compute an image of the mandelbrot set in parallel
///using the standard library with unsafe functions.

/// # Arguments
///
/// * `bounds` - The length and width of the image
/// * `upper_left` - A Complex Number specifying the upper_left point on the complex lane.
/// * `lower_right` - A Complex Number specifying the lower_right point on the complex lane.
/// * `number_of_threads` - The number of threads and at the same time the number of chunks.
pub fn time_fork_join_unsafe(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    number_of_threads: usize,
) -> Result<f64, CustomError> {
    let arr_size = bounds.0 * bounds.1;
    let v = vec![0 as u8; arr_size];
    //Inhibit the compiler from automatically call the destructor to gain full control of v.
    let mut v = mem::ManuallyDrop::new(v);
    //create a Raw Pointer of v
    let p: *mut u8 = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let pixels = Arc::new(Wrapper(UnsafeCell::new(p)));
    //Round the count upward to make sure that the bands cover the entire image.
    let rows_per_band = bounds.1 / number_of_threads + 1;
    let chunk_size = rows_per_band * bounds.0;
    let mut threads = vec![];

    let mut start = MyTimestamp::new();
    let mut end = MyTimestamp::new();

    start.gettime(Clock::ClockMonotonicRaw)?;
    //Iterate over arr_size in steps to create selfmade chunks.
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
        //Rebuild the vector from Raw pointer.
        let rebuilt = Vec::from_raw_parts(*pixels.0.get(), len, cap);
        crate::mandel::write_image("mandel.png", &rebuilt, bounds)?;
    }

    //Call the destructor for pixels
    drop(v);
    Ok(start.compute_time_millis(end))
}
