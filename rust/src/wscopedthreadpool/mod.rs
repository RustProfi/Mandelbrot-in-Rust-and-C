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
/// * `bounds` - A pair giving the width and height of the image in pixels.
/// * `upper_left` - The upper left point on the complex plane designating the area of the image.
/// * `lower_right` - The lower right point on the complex plane designating the area of the image.
/// * `rows_per_band` - The number of rows per band divided by the height gives the number of bands.
/// * `pool_size` - The number of threads the threadpool will be initialized with.
/// * `draw` - Decides whether to write the computed mandelbrot set to png or not.
pub fn time_scoped_threadpool(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    rows_per_band: usize,
    pool_size: usize,
    draw: bool,
) -> Result<f64, CustomError> {
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    let mut start = MyTimestamp::new();
    let mut end = MyTimestamp::new();

    start.gettime(Clock::ClockMonotonicRaw)?;
    let mut pool = Pool::new(pool_size as u32);
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

///Measures how long it takes for every number of rows per band in a
///range from 1 to 80 by repeating the measurement 20 times each. The results are written to a file.

/// # Arguments
///
/// * `bounds` - A pair giving the width and height of the image in pixels.
/// * `upper_left` - The upper left point on the complex plane designating the area of the image.
/// * `lower_right` - The lower right point on the complex plane designating the area of the image.
/// * `pool_size` - The number of threads the threadpool will be initialized with.
/// * `draw` - Decides whether to write the computed mandelbrot set to png or not.
pub fn measure_workload_scoped_threadpool(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    pool_size: usize,
    draw: bool,
) -> Result<(), CustomError> {
    let mut file = File::create("rust_scoped_threadpool_performance.txt")?;

    for rows_per_band in 1..=80 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_scoped_threadpool(
                bounds,
                upper_left,
                lower_right,
                rows_per_band,
                pool_size,
                draw,
            )?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", rows_per_band, time))?;
    }
    Ok(())
}
