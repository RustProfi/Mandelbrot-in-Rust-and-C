extern crate num;
mod customerror;
mod mandel;
mod time;
mod wcrossbeam;
mod wrayon;
mod wscopedthreadpool;
mod wthreads;
mod wthreadsunsafe;
use mandelbrot::measure::{
    measure_workload_crossbeam, measure_workload_rayon, measure_workload_scoped_threadpool,
    measure_workload_threads, measure_workload_threads_unsafe,
};
use num::Complex;
use std::process::exit;
use wcrossbeam::time_with_crossbeam;
use wrayon::time_with_rayon;
use wscopedthreadpool::time_with_scoped_threadpool;
use wthreads::time_threads;
use wthreadsunsafe::time_threads_unsafe;

static BOUNDS: (usize, usize) = (5000, 5000);
static NTHREADS: usize = 50;
static ROWS_PER_BAND: usize = 5;
static DRAW: bool = true;
static UPPER_LEFT: Complex<f64> = Complex { re: -1.6, im: 1.2 };
static LOWER_RIGHT: Complex<f64> = Complex { re: 0.6, im: -1.2 };

///A basic tui with error handling
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || args.len() > 4 {
        eprintln!("Usage: ./mandelbrot <Method> [args]");
        eprintln!("Methods: threads|th, threadsunsafe|tu, crossbeam|cb, scoped_threadpool|st, rayon|ra, all>");
        eprintln!("args: -m (Performance measure)");
        std::process::exit(1);
    }

    if args.iter().any(|x| x.eq("threads"))
        || args.iter().any(|x| x.eq("th"))
        || args.iter().any(|x| x.eq("all"))
    {
        if args.iter().any(|x| x.eq("-m")) {
            match measure_workload_threads(BOUNDS, UPPER_LEFT, LOWER_RIGHT) {
                Ok(_) => println!("Workload measure with threading complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_threads(BOUNDS, UPPER_LEFT, LOWER_RIGHT, NTHREADS, DRAW) {
                Ok(time) => println!("Time with threading: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }
     if args.iter().any(|x| x.eq("threadsunsafe"))
        || args.iter().any(|x| x.eq("tu"))
        || args.iter().any(|x| x.eq("all"))
    {
        if args.iter().any(|x| x.eq("-m")) {
            match measure_workload_threads_unsafe(BOUNDS, UPPER_LEFT, LOWER_RIGHT) {
                Ok(_) => println!("Workload measure with threading unsafe complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_threads_unsafe(BOUNDS, UPPER_LEFT, LOWER_RIGHT, NTHREADS, DRAW) {
                Ok(time) => println!("Time with threading unsafe: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }
     if args.iter().any(|x| x.eq("crossbeam"))
        || args.iter().any(|x| x.eq("cb"))
        || args.iter().any(|x| x.eq("all"))
    {
        if args.iter().any(|x| x.eq("-m")) {
            match measure_workload_crossbeam(BOUNDS, UPPER_LEFT, LOWER_RIGHT) {
                Ok(_) => println!("Workload measure with crossbeam complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_with_crossbeam(BOUNDS, UPPER_LEFT, LOWER_RIGHT, NTHREADS, DRAW) {
                Ok(time) => println!("Time with crossbeam: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }
     if args.iter().any(|x| x.eq("scoped_threadpool"))
        || args.iter().any(|x| x.eq("st"))
        || args.iter().any(|x| x.eq("all"))
    {
        if args.iter().any(|x| x.eq("-m")) {
            match measure_workload_scoped_threadpool(BOUNDS, UPPER_LEFT, LOWER_RIGHT, 8) {
                Ok(_) => println!("Workload measure with scoped_threadpool complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_with_scoped_threadpool(
                BOUNDS,
                UPPER_LEFT,
                LOWER_RIGHT,
                ROWS_PER_BAND,
                8,
                DRAW,
            ) {
                Ok(time) => println!("Time with scoped_threadpool: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }
     if args.iter().any(|x| x.eq("rayon"))
        || args.iter().any(|x| x.eq("ra"))
        || args.iter().any(|x| x.eq("all"))
    {
        if args.iter().any(|x| x.eq("-m")) {
            match measure_workload_rayon(BOUNDS, UPPER_LEFT, LOWER_RIGHT) {
                Ok(_) => println!("Workload measure with rayon complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_with_rayon(BOUNDS, UPPER_LEFT, LOWER_RIGHT, ROWS_PER_BAND, DRAW) {
                Ok(time) => println!("Time with rayon: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }
}
