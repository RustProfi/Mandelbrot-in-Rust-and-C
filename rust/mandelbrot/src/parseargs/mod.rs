use crate::customerror::CustomError;

use num::Complex;
use clap::{Arg, App, SubCommand};

//defaults
static BOUNDS: (usize, usize) = (5000, 5000);
static NTHREADS: usize = 50;
static ROWS_PER_BAND: usize = 5;
static DRAW: bool = true;
static UPPER_LEFT: Complex<f64> = Complex { re: -1.6, im: 1.2 };
static LOWER_RIGHT: Complex<f64> = Complex { re: 0.6, im: -1.2 };

pub struct ParsedArgs {
    pub mechanism: String,
    pub measure: bool,
    pub bounds: (usize, usize),
    pub threads: usize,
    pub rows_per_band: usize,
    pub draw: bool,
    pub upper_left: Complex<f64>,
    pub lower_right: Complex<f64>
}

pub fn parsearguments() -> Result<ParsedArgs, CustomError> {
    let matches = clap_app!(Mandelbrot =>
        (version: "1.0")
        (author: "Marno Janetzky <janetzkymarno@gmail.com>")
        (about: "Computes an image of the Mandelbrot set. There is a default value for each unspecified option.")
        (@arg Mechanism: +takes_value +required -m --mechanism "Mechanisms can be: all, threads|th, threadsunsafe|tu, crossbeam|cb, scoped_threadpool|st, rayon|ra")
        (@arg Measure: -w --workload "Measures the workload and writes the results to a file. Hint: Consider DrawOff")
        (@arg Bounds: +takes_value -b --bounds "Set the width and heigth of the image in pixel. Example: 5000,5000")
        (@arg Threads: +takes_value -t --threads "Specify the number of threads. Hint: The rayon mechanism doesn't care about threads")
        (@arg Rows_per_band: +takes_value -r --rows "Specify the rows per band. Hint: Only necessary for scoped_threadpool and rayon")
        (@arg DrawOff: +takes_value -d --drawoff "Disables writing the image to a png file")
        (@arg ComplexCoords: +takes_value -c --complexcoords "Specify an upper left and a lower right point on the complex plane. Example: For upper left = -1.6 + 1.2 * I and lower right = 0.6 - 1.2 * I, enter -1.6,1.2,0.6,-1.2")
    ).get_matches();

    let mechanism = matches.value_of("Mechanism").unwrap();

    let mut bounds = BOUNDS;

    Ok(ParsedArgs{mechanism: mechanism.to_string(), measure: false, bounds: BOUNDS, threads: 50, rows_per_band:1, draw: true, upper_left: UPPER_LEFT, lower_right: LOWER_RIGHT})

    /* match matches.value_of("Bounds").unwrap() {
        Some(value) =>  {
        let split: Vec<&str> = matches.value_of("Bounds").unwrap().split(",").collect();
        bounds = (split[0].parse::<usize>()?, split[1].parse::<usize>()?);
        },
        None => BOUNDS
    };*/
}
