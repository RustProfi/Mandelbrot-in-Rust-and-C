extern crate crossbeam;
use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render};
use crate::time::{Clock, MyTimer};
use num::Complex;

pub fn time_with_crossbeam(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<f64, CustomError> {
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let number_of_threads = 8;
    let rows_per_band = bounds.1 / number_of_threads + 1;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    let mut start = MyTimer::new();
    let mut end = MyTimer::new();

    start.gettime(Clock::ClockMonotonicRaw)?;

    //Fehlertyp std::convert::From<std::boxed::Box<dyn std::any::Any + std::marker::Send>>
    //lässt sich nicht mit einem From trait in CustomError umformen
    //und auch mit dem failure crate, welches einen Wrapper um "jeden" Fehlertyp
    //macht. In diesem Fall kann es das aber auch nicht, weil die size des Errors zur
    //Kompilierzeit nicht bekannt ist. Hier hat Rust noch eine Lücke zu füllen.

    crossbeam::scope(|spawner| {
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
    });

    end.gettime(Clock::ClockMonotonicRaw)?;
    crate::mandel::write_image("mandel.png", &pixels, bounds)?;
    Ok(start.compute_time_millis(end))
}
