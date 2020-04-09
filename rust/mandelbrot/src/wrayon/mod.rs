extern crate rayon;
use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render};
use crate::time::{Clock, MyTimestamp};
use num::Complex;
use rayon::prelude::*;

///Measure in ms how long it takes to compute an image of the mandelbrot set in parallel
///using the rayon crate.

/// # Arguments
///
/// * `bounds` - The length and width of the image
/// * `upper_left` - A Complex Number specifying the upper_left point on the complex lane.
/// * `lower_right` - A Complex Number specifying the lower_right point on the complex lane.
/// * `rows_per_band` - The number of rows per band.
pub fn time_with_rayon(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<f64, CustomError> {
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let rows_per_band = 5;

    let mut start = MyTimestamp::new();
    let mut end = MyTimestamp::new();

    start.gettime(Clock::ClockMonotonicRaw)?;
    pixels
        .par_chunks_mut(rows_per_band * bounds.0)
        .into_par_iter()
        .enumerate()
        .for_each(|(i, band)| {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });

    end.gettime(Clock::ClockMonotonicRaw)?;
    //crate::mandel::write_image("mandel.png", &pixels, bounds)?;
    Ok(start.compute_time_millis(end))
}
