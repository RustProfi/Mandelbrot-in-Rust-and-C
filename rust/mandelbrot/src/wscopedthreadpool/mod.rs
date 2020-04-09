extern crate scoped_threadpool;
use crate::customerror::CustomError;
use crate::mandel::{pixel_to_point, render};
use crate::time::{Clock, MyTimer};
use num::Complex;
use scoped_threadpool::Pool;

pub fn time_with_scoped_threadpool(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    pool: usize,
) -> Result<f64, CustomError> {
    let mut pixels = vec![0; bounds.0 * bounds.1];
    let mut pool = Pool::new(pool as u32);

    let rows_per_band = 5;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    let mut start = MyTimer::new();
    let mut end = MyTimer::new();

    start.gettime(Clock::ClockMonotonicRaw)?;

    pool.scoped(|scope| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            scope.execute(move || {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    });

    end.gettime(Clock::ClockMonotonicRaw)?;
    //crate::mandel::write_image("mandel.png", &pixels, bounds)?;
    Ok(start.compute_time_millis(end))
}
