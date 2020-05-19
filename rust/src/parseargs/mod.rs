use crate::customerror::CustomError;
use num::Complex;

///defaults
static BOUNDS: (usize, usize) = (5000, 5000);
static NTHREADS: usize = 8;
static ROWS_PER_BAND: usize = 1;
static UPPER_LEFT: Complex<f64> = Complex { re: -1.6, im: 1.2 };
static LOWER_RIGHT: Complex<f64> = Complex { re: 0.6, im: -1.2 };

///Holds the parsed or default values
pub struct ParsedArgs {
    pub mechanism: String,
    pub measure: bool,
    pub bounds: (usize, usize),
    pub threads: usize,
    pub rows_per_band: usize,
    pub draw: bool,
    pub upper_left: Complex<f64>,
    pub lower_right: Complex<f64>,
}

///A fancy cli powered by the clap crate. There is a default value for each unspecified option.
///Run with --help for more information
pub fn parse_arguments() -> Result<ParsedArgs, CustomError> {
    let matches = clap_app!(Mandelbrot =>
        (version: "1.0")
        (author: "Marno Janetzky <janetzkymarno@gmail.com>")
        (about: "Computes an image of the Mandelbrot set. There is a default value for each unspecified option.")
        (@arg Mechanism: +takes_value +required -m --mechanism "Mechanisms may be: all, threads|th, threadsunsafe|tu, crossbeam|cb, scoped_threadpool|st, rayon|ra")
        (@arg Measure: -w --workload "Measures the workload and writes the results to a file. Hint: Consider DrawOff while measuring")
        (@arg Bounds: +takes_value -b --bounds "Set the width and heigth of the image in pixel. Example: 5000,5000")
        (@arg Threads: +takes_value -t --threads "Specify the number of threads. Hint: The rayon mechanism doesn't care about threads")
        (@arg Rows_per_band: +takes_value -r --rows "Specify the rows per band. Hint: Only necessary for scoped_threadpool and rayon")
        (@arg DrawOff: -d --drawoff "Disables writing the image to a png file")
        //Unfortunately, this has to be written in a row, otherwise it will mess up the formatting
        (@arg ComplexCoords: +takes_value -c --complexcoords "Specify an upper left and a lower right point on the complex plane.\nAttention: Enter a leading ',' because otherwise clap will interpret a '-' as a unknown argument.\nExample: For upper left = -1.6 + 1.2 * I and lower right = 0.6 - 1.2 * I, enter: ,-1.6,1.2,0.6,-1.2")
    ).get_matches();

    let mechanism = matches.value_of("Mechanism").unwrap();

    if !(mechanism.eq("all")
        || mechanism.eq("threads")
        || mechanism.eq("th")
        || mechanism.eq("threadsunsafe")
        || mechanism.eq("tu")
        || mechanism.eq("crossbeam")
        || mechanism.eq("cb")
        || mechanism.eq("scoped_threadpool")
        || mechanism.eq("st")
        || mechanism.eq("rayon")
        || mechanism.eq("ra"))
    {
        return Err(CustomError::InvalidArgument);
    }

    let bounds = match matches.value_of("Bounds") {
        Some(v) => {
            let split: Vec<&str> = v.split(',').collect();
            if split.len() < 2 {
                return Err(CustomError::InvalidArgument);
            }
            (split[0].parse::<usize>()?, split[1].parse::<usize>()?)
        }
        None => BOUNDS,
    };

    let threads = match matches.value_of("Threads") {
        Some(v) => v.parse::<usize>()?,
        None => NTHREADS,
    };

    let rows_per_band = match matches.value_of("Rows_per_band") {
        Some(v) => v.parse::<usize>()?,
        None => ROWS_PER_BAND,
    };

    let (upper_left, lower_right) = match matches.value_of("ComplexCoords") {
        Some(v) => {
            let split: Vec<&str> = v.split(',').collect();
            if split.len() < 5 {
                return Err(CustomError::InvalidArgument);
            }
            let ul = Complex {
                re: split[1].parse::<f64>()?,
                im: split[2].parse::<f64>()?,
            };
            let ur = Complex {
                re: split[3].parse::<f64>()?,
                im: split[4].parse::<f64>()?,
            };
            (ul, ur)
        }
        None => (UPPER_LEFT, LOWER_RIGHT),
    };

    Ok(ParsedArgs {
        mechanism: mechanism.to_string(),
        measure: matches.is_present("Measure"),
        bounds,
        threads,
        rows_per_band,
        draw: !matches.is_present("DrawOff"),
        upper_left,
        lower_right,
    })
}
