use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render_threads_unsafe, write_image};
use crate::time::{Clock, MyTimestamp};
use num::Complex;
use std::cell::UnsafeCell;
use std::fs::File;
use std::io::prelude::*;
use std::mem;
use std::sync::Arc;
use std::thread;

///A wrapper around UnsafeCell<T> to be able to implement Send and Sync
pub struct WrappedUnsafeCell<T>(pub UnsafeCell<T>);
unsafe impl<T> Send for WrappedUnsafeCell<T> {}
unsafe impl<T> Sync for WrappedUnsafeCell<T> {}

///Measure in ms how long it takes to compute an image of the mandelbrot set in parallel
///using the standard library with unsafe functions.

/// # Arguments
///
/// * `bounds` - A pair giving the width and height of the image in pixels.
/// * `upper_left` - The upper left point on the complex plane designating the area of the image.
/// * `lower_right` - The lower right point on the complex plane designating the area of the image.
/// * `number_of_threads` - The number of threads gives the number of chunks.
/// * `draw` - Decides whether to write the computed mandelbrot set to png or not.
pub fn time_threads_unsafe(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    number_of_threads: usize,
    draw: bool,
) -> Result<f64, CustomError> {
    let arr_len = bounds.0 * bounds.1;
    let vec = vec![0 as u8; arr_len];
    //Inhibit the compiler from automatically call the destructor to gain full control of v.
    let mut vec = mem::ManuallyDrop::new(vec);
    //create a Raw Pointer of v
    let p: *mut u8 = vec.as_mut_ptr();
    let len = vec.len();
    let cap = vec.capacity();

    let pixels = Arc::new(WrappedUnsafeCell(UnsafeCell::new(p)));
    //if number_of_threads doesn't fit perfectly in height without rest, it must be round upward to make sure that the bands cover the entire image.
    let rows_per_band = if bounds.1 % number_of_threads == 0 {
        bounds.1 / number_of_threads
    } else {
        bounds.1 / number_of_threads + 1
    };

    let chunk_len = rows_per_band * bounds.0;
    let mut threads = vec![];

    let mut start = MyTimestamp::new();
    let mut end = MyTimestamp::new();

    start.gettime(Clock::ClockMonotonicRaw)?;
    //Iterate over arr_size in steps to create selfmade chunks.
    for (i, offset) in (0..arr_len).step_by(chunk_len).enumerate() {
        let pixels_ref = pixels.clone();
        //The last chunk can be smaller than the other chunks
        let check_chunk_len = if arr_len - offset > chunk_len {
            chunk_len
        } else {
            arr_len - offset
        };
        let top = rows_per_band * i;
        let height = check_chunk_len / bounds.0;
        let band_bounds = (bounds.0, height);
        let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right =
            pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
        threads.push(thread::spawn(move || {
            render_threads_unsafe(
                pixels_ref,
                offset,
                band_bounds,
                band_upper_left,
                band_lower_right,
            );
        }));
    }

    for thread in threads {
        if thread.join().is_err() {
            return Err(CustomError::ThreadPanic);
        };
    }
    end.gettime(Clock::ClockMonotonicRaw)?;

    unsafe {
        if draw {
            //Rebuild the vector from Raw pointer. In this case vec will automatically be dropped
            let rebuilt = Vec::from_raw_parts(*pixels.0.get(), len, cap);
            write_image("mandel.png", &rebuilt, bounds)?;
        } else {
            //if there was no rebuild from raw parts it must be manually be dropped.
            mem::ManuallyDrop::drop(&mut vec);
        }
    }

    Ok(start.compute_time_millis(end))
}

///Measures how long it takes for every number of threads in a
///range from 4 to 80 by repeating the measurement 20 times each. The results are written to a file.

/// # Arguments
///
/// * `bounds` - A pair giving the width and height of the image in pixels.
/// * `upper_left` - The upper left point on the complex plane designating the area of the image.
/// * `lower_right` - The lower right point on the complex plane designating the area of the image.
/// * `draw` - Decides whether to write the computed mandelbrot set to png or not.
pub fn measure_workload_threads_unsafe(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    draw: bool,
) -> Result<(), CustomError> {
    let mut file = File::create("rust_threads_unsafe_performance.txt")?;

    for thread_count in 4..=80 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_threads_unsafe(bounds, upper_left, lower_right, thread_count, draw)?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", thread_count, time))?;
    }
    Ok(())
}
