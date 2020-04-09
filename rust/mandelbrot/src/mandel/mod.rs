extern crate image;
extern crate num;

use crate::customerror::CustomError;
use crate::wforkjoinunsafe::Wrapper;
use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::ptr;
use std::sync::Arc;
use std::sync::Mutex;

/// Try to determine if `c` is in the Mandelbrot set, using at most 255
/// iterations to decide due to the color spectrum of the Png Writer. For
/// a more precise estimation this value must be increased.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_mandel_time(c: Complex<f64>) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..255 {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64, // Why subtraction here? pixel.1 increases as we go down,
                                                                       // but the imaginary component increases as we go up.
    }
}
#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

///Render a rectangle of the Mandelbrot set into a buffer of pixels.
///The `bounds` argument gives the
///which holds one grayscale pixel
///arguments specify points on the
///left and lower-right corners of
///width and height of the buffer `pixels`,
///per byte. The `upper_left` and `lower_right`
///complex plane corresponding to the upper-
///the pixel buffer.

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    if !(pixels.len() == bounds.0 * bounds.1) {
        return Err(CustomError::UnfittingArray);
    }

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_mandel_time(point) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
    Ok(())
}

pub fn render_fork_join(
    pixels: Arc<Mutex<Vec<u8>>>,
    offset: usize,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    if offset + bounds.0 * bounds.1 > pixels.lock().unwrap().len() {
        return Err(CustomError::UnfittingArray);
    }
    //Check for every pixel wether it is in the mandelbrot set or not.
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels.lock().unwrap()[offset + (row * bounds.0 + column)] =
                match escape_mandel_time(point) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
        }
    }
    Ok(())
}

pub fn render_fork_join_unsafe(
    pixels: Arc<Wrapper<*mut u8>>,
    offset: usize,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    unsafe {
        let ptr = *pixels.0.get();

        //Check for every pixel wether it is in the mandelbrot set or not.
        for row in 0..bounds.1 {
            for column in 0..bounds.0 {
                let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

                let c = match escape_mandel_time(point) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
                ptr::write(
                    ptr.offset(offset as isize + (row * bounds.0 + column) as isize),
                    c,
                );
            }
        }
    }
    Ok(())
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), CustomError> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)?;
    Ok(())
}
