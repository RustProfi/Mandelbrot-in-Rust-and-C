extern crate csv;
extern crate num;
mod customerror;
mod mandel;
mod time;
mod wcrossbeam;
mod wforkjoin;
mod wforkjoinunsafe;
mod wrayon;
mod wscopedthreadpool;
//use csv::Writer;
use num::Complex;
use std::process::exit;
use wcrossbeam::time_with_crossbeam;
use wforkjoin::time_fork_join;
use wforkjoinunsafe::time_fork_join_unsafe;
use wrayon::time_with_rayon;
use wscopedthreadpool::time_with_scoped_threadpool;

static BOUNDS: (usize, usize) = (5000, 5000);
static UPPER_LEFT: Complex<f64> = Complex { re: -1.6, im: 1.2 };
static LOWER_RIGHT: Complex<f64> = Complex { re: 0.6, im: -1.2 };

//Todo alles kommentieren und überlegen wo unwrap sinn macht und wo nicht
//Todo schönes error handling

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 && !(args.len() == 3 && args[2].eq("--release")) {
        eprintln!("Usage: mandelbrot <method>");
        eprintln!("Methods: forkjoin|fj, crossbeam|cb, scoped_threadpool|st all>");
        std::process::exit(1);
    }

    //10x verschiedene Threadanzahl messen
    if args[1].eq("forkjoin") || args[1].eq("fj") || args[1].eq("all") {
        match time_fork_join(BOUNDS, UPPER_LEFT, LOWER_RIGHT, 8) {
            Ok(time) => println!("Fork join variation 2 compution time: {}ms", time),
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    }

    if args[1].eq("forkjoinunsafe") || args[1].eq("fu") || args[1].eq("all") {
        match time_fork_join_unsafe(BOUNDS, UPPER_LEFT, LOWER_RIGHT, 8) {
            Ok(time) => println!("Fork join variation 2 compution time: {}ms", time),
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    }

    //10x verschiedene Threadanzahl messen
    if args[1].eq("crossbeam") || args[1].eq("cb") || args[1].eq("all") {
        match time_with_crossbeam(BOUNDS, UPPER_LEFT, LOWER_RIGHT, 8) {
            Ok(time) => println!("Crossbeam compution time: {}ms", time),
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    }

    //10x verschiedene row anzahl
    if args[1].eq("scoped_threadpool") || args[1].eq("st") || args[1].eq("all") {
        let mut vecofvec = vec![vec![]; 10];

        for _ in 0..10 {
            for (i, x) in (1..300).step_by(30).enumerate() {
                println!("{}", x);
                match time_with_scoped_threadpool(BOUNDS, UPPER_LEFT, LOWER_RIGHT, x) {
                    Ok(time) => {
                        println!("scoped_threadpool compution time: {}ms", time);
                        vecofvec[i].push(time);
                    }
                    Err(e) => {
                        println!("{}", e);
                        exit(1);
                    }
                }
            }
        }

        for (i, xd) in vecofvec.iter().enumerate() {
            let sum: f64 = xd.iter().sum();
            println!("{}: {}ms", i, sum / 10.0);
        }
    }

    if args[1].eq("rayon") || args[1].eq("ra") || args[1].eq("all") {
        match time_with_rayon(BOUNDS, UPPER_LEFT, LOWER_RIGHT, 5) {
            Ok(time) => println!("rayon compution time: {}ms", time),
            Err(e) => {
                println!("{}", e);
                exit(1);
            }
        }
    }
}
