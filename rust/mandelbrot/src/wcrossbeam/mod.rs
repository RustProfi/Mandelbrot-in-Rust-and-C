extern crate crossbeam;
use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render, write_image};
use crate::time::{Clock, MyTimestamp};
use num::Complex;

///Measure in ms how long it takes to compute an image of the mandelbrot set in parallel using
//the crossbeam crate.

/// # Arguments
///
/// * `bounds` - The width and height of the image
/// * `upper_left` - A Complex Number specifying the upper_left point on the complex lane.
/// * `lower_right` - A Complex Number specifying the lower_right point on the complex lane.
/// * `number_of_threads` - The number of threads and at the same time the number of chunks.
pub fn time_with_crossbeam(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    number_of_threads: usize,
    draw: bool,
) -> Result<f64, CustomError> {
    let mut pixels = vec![0; bounds.0 * bounds.1];
    //if rows_per_band doesn't fit perfectly in pixels_len without rest, it must be round upward to make sure that the bands cover the entire image.
    let rows_per_band = if (bounds.0 * bounds.1) % (bounds.1 / number_of_threads) == 0 {
        bounds.1 / number_of_threads
    } else {
        bounds.1 / number_of_threads + 1
    };
    //Get non overlapping bands of the image.
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    let mut start = MyTimestamp::new();
    let mut end = MyTimestamp::new();

    start.gettime(Clock::ClockMonotonicRaw)?;

    //Todo: Englisch
    //Fehlertyp std::convert::From<std::boxed::Box<dyn std::any::Any + std::marker::Send>>
    //lässt sich nicht mit einem From trait in CustomError umformen
    //und auch mit dem failure crate, welches einen Wrapper um "jeden" Fehlertyp
    //macht. In diesem Fall kann es das aber auch nicht, weil die size des Errors zur
    //Kompilierzeit nicht bekannt ist. Hier hat Rust noch eine Lücke zu füllen.
    match crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            spawner.spawn(move |_| -> Result<(), CustomError> {
                render(band, band_bounds, band_upper_left, band_lower_right)?;
                Ok(())
            });
        }
    }) {
        Ok(_) => {}
        Err(_) => return Err(CustomError::CrossbeamError),
    }

    end.gettime(Clock::ClockMonotonicRaw)?;
    if draw {
        write_image("mandel.png", &pixels, bounds)?;
    }
    Ok(start.compute_time_millis(end))
}
