extern crate scoped_threadpool;
use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render, write_image};
use crate::time::{Clock, MyTimestamp};
use num::Complex;
use scoped_threadpool::Pool;
use std::fs::File;
use std::io::prelude::*;

///Measure in ms how long it takes to compute an image of the mandelbrot set in parallel
///using the scoped_threadpool crate.

/// # Arguments
///
/// * `bounds` - The length and width of the image
/// * `upper_left` - A Complex Number specifying the upper_left point on the complex lane.
/// * `lower_right` - A Complex Number specifying the lower_right point on the complex lane.
/// * `rows_per_band` - The number of rows per band.
pub fn time_with_scoped_threadpool(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    rows_per_band: usize,
    number_of_threads: u32,
    draw: bool,
) -> Result<f64, CustomError> {
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    let mut start = MyTimestamp::new();
    let mut end = MyTimestamp::new();

    start.gettime(Clock::ClockMonotonicRaw)?;
    let mut pool = Pool::new(number_of_threads);
    pool.scoped(|scope| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            scope.execute(move || {
                render(band, band_bounds, band_upper_left, band_lower_right).unwrap();
            });
        }
    });

    end.gettime(Clock::ClockMonotonicRaw)?;
    if draw {
        write_image("mandel.png", &pixels, bounds)?;
    }
    Ok(start.compute_time_millis(end))
}

///Measures for a given base and bounds how long it takes for every number of rows per band in a
///range from 1 to 80 by repeating the measurement 20 times each. The results are written to a file.

/// # Arguments
///
/// * `bounds` - The width and height of the image
/// * `upper_left` - A Complex Number specifying the upper_left point on the complex lane.
/// * `lower_right` - A Complex Number specifying the lower_right point on the complex lane.
/// * `number_of_threads` - The number of threads the threadpool will be initialized with
pub fn measure_workload_scoped_threadpool(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    number_of_threads: u32,
) -> Result<(), CustomError> {
    let mut file = File::create("rust_scoped_threadpool_performance.txt")?;

    for rows_per_band in 1..=80 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_with_scoped_threadpool(
                bounds,
                upper_left,
                lower_right,
                rows_per_band,
                number_of_threads,
                false,
            )?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", rows_per_band, time))?;
    }
    Ok(())
}
