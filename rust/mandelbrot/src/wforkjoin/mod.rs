use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render_fork_join};
use crate::time::{Clock, MyTimestamp};
use num::Complex;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

///Measure in ms how long it takes to compute an image of the mandelbrot set in parallel using the
///standard library only.

/// # Arguments
///
/// * `bounds` - The length and width of the image
/// * `upper_left` - A Complex Number specifying the upper_left point on the complex lane.
/// * `lower_right` - A Complex Number specifying the lower_right point on the complex lane.
/// * `number_of_threads` - The number of threads and at the same time the number of chunks.
pub fn time_fork_join(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    number_of_threads: usize,
) -> Result<f64, CustomError> {
    let arr_len = bounds.0 * bounds.1;
    let pixels: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; arr_len]));
    //if rows_per_band doesn't fit perfectly in pixels_len without rest, it must be round upward to make sure that the bands cover the entire image.
    let rows_per_band = if (bounds.0 * bounds.1) % (bounds.1 / number_of_threads) == 0 {
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
        threads.push(thread::spawn(move || -> Result<(), CustomError> {
            render_fork_join(
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

    crate::mandel::write_image("mandel.png", &*pixels.lock().unwrap(), bounds)?;
    Ok(start.compute_time_millis(end))
}
