use crate::customerror::CustomError;
use crate::wthreadsunsafe::WrappedUnsafeCell;
use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::ptr;
use std::sync::Arc;
use std::sync::Mutex;

/// Try to determine if `c` is in the Mandelbrot set, using at most 256
/// iterations due to the grayscale color spectrum of the Png Writer.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.

/// # Arguments
///
/// * `c` - A complex number to be determined if it is in the mandelbrot set or not.
fn escape_mandel_iterations(c: Complex<f64>) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..256 {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.

/// # Arguments
///
/// * `bounds` - A pair giving the width and height of the image in pixels.
/// * `pixel` - A (column, row) pair indicating a particular pixel in that image.
/// * `upper_left` - The upper left point on the complex plane designating the area of the image.
/// * `lower_right` - The lower right point on the complex plane designating the area of the image.
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
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

///Render a rectangle of the Mandelbrot set into a buffer of pixels.

/// # Arguments
///
/// * `pixels` - A buffer which holds one grayscale pixel per byte.
/// * `bounds` - A pair giving the width and height of the buffer.
/// * `upper_left` - The upper left point on the complex plane corresponding to upper left corner of the buffer.
/// * `lower_right` - The lower right point on the complex plane corresponding to lower right corner of the buffer.
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    if pixels.len() != bounds.0 * bounds.1 {
        return Err(CustomError::UnfittingArray);
    }

    //Check for every pixel wether it is in the mandelbrot set or not.
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_mandel_iterations(point) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
    Ok(())
}

///Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
///This is a modification of the render function that can be used safely between threads without
///having to use an external crate which provides a scope environment.

/// # Arguments
///
/// * `pixels` - A buffer the size of the image which holds one grayscale pixel per byte.
/// * `offset` - An offset which specify which "band" of buffer will be mutated.
/// * `bounds` - A pair giving the width and height of the band.
/// * `upper_left` - The upper left point on the complex plane corresponding to upper left corner of the band.
/// * `lower_right` - The lower right point on the complex plane corresponding to lower right corner of the band.
pub fn render_threads(
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
            let iterations = match escape_mandel_iterations(point) {
                None => 0,
                Some(count) => 255 - count as u8,
            };

            //Assuming no thread will panic
            let mut guard = pixels.lock().unwrap();
            guard[offset + (row * bounds.0 + column)] = iterations;
        }
    }
    Ok(())
}

///Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
///This is a modification of the render function that can be used safely between threads without
///having to use an external crate which provides a scope environment. In Addititon this function
///uses unsafe code so no locking mechanism is used.

/// # Arguments
///
/// * `pixels` - A buffer the size of the image which holds one grayscale pixel per byte.
/// * `offset` - An offset which specify which "band" of buffer will be mutated.
/// * `bounds` - A pair giving the width and height of the band.
/// * `upper_left` - The upper left point on the complex plane corresponding to upper left corner of the band.
/// * `lower_right` - The lower right point on the complex plane corresponding to lower right corner of the band.
pub fn render_threads_unsafe(
    pixels: Arc<WrappedUnsafeCell<*mut u8>>,
    offset: usize,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    unsafe {
        //The get() function returns a *mut T pointer which needs to be dereferenced
        //to get the Pointer.
        let ptr = *pixels.0.get();

        //Check for every pixel wether it is in the mandelbrot set or not.
        for row in 0..bounds.1 {
            for column in 0..bounds.0 {
                let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

                let mandel_time = match escape_mandel_iterations(point) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
                ptr::write(
                    ptr.offset(offset as isize + (row * bounds.0 + column) as isize),
                    mandel_time,
                );
            }
        }
    }
}

/// Write an image to a png file

/// # Arguments
///
/// * `filename` - The name of the image which will be created.
/// * `pixels` - A buffer holding one pixel per byte in grayscale.
/// * `bounds` - The dimensions of the image.
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
