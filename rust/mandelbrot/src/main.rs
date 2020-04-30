use std::process::exit;
use mandelbrot::parseargs::{parsearguments};
use mandelbrot::wcrossbeam::{measure_workload_crossbeam, time_crossbeam};
use mandelbrot::wrayon::{measure_workload_rayon, time_rayon};
use mandelbrot::wscopedthreadpool::{measure_workload_scoped_threadpool, time_scoped_threadpool};
use mandelbrot::wthreads::{measure_workload_threads, time_threads};
use mandelbrot::wthreadsunsafe::{measure_workload_threads_unsafe, time_threads_unsafe};

///A basic tui with error handling
fn main() {

    let args = match parsearguments() {
        Ok(k) => k,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    if args.mechanism.eq("threads") || args.mechanism.eq("th") || args.mechanism.eq("all") {
        if args.measure {
            match measure_workload_threads(args.bounds, args.upper_left, args.lower_right) {
                Ok(_) => println!("Workload measure with threading complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_threads(args.bounds, args.upper_left, args.lower_right, args.threads, args.draw) {
                Ok(time) => println!("Time with threading: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }

    if args.mechanism.eq("threadsunsafe") || args.mechanism.eq("tu") || args.mechanism.eq("all") {
        if args.measure {
            match measure_workload_threads_unsafe(args.bounds, args.upper_left, args.lower_right) {
                Ok(_) => println!("Workload measure with threading unsafe complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_threads_unsafe(args.bounds, args.upper_left, args.lower_right, args.threads, args.draw) {
                Ok(time) => println!("Time with threading unsafe: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }

    if args.mechanism.eq("crossbeam") || args.mechanism.eq("cb") || args.mechanism.eq("all") {
        if args.measure {
            match measure_workload_crossbeam(args.bounds, args.upper_left, args.lower_right) {
                Ok(_) => println!("Workload measure with crossbeam complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_crossbeam(args.bounds, args.upper_left, args.lower_right, args.threads, args.draw) {
                Ok(time) => println!("Time with crossbeam: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }

    if args.mechanism.eq("scoped_threadpool") ||args. mechanism.eq("st") || args.mechanism.eq("all") {
        if args.measure {
            match measure_workload_scoped_threadpool(args.bounds, args.upper_left, args.lower_right, args.threads) {
                Ok(_) => println!("Workload measure with scoped_threadpool complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_scoped_threadpool(args.bounds, args.upper_left, args.lower_right, args.rows_per_band, args.threads, args.draw) {
                Ok(time) => println!("Time with scoped_threadpool: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }

    if args.mechanism.eq("rayon") || args.mechanism.eq("ra") ||args. mechanism.eq("all") {
        if args.measure {
            match measure_workload_rayon(args.bounds, args.upper_left, args.lower_right) {
                Ok(_) => println!("Workload measure with rayon complete!"),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        } else {
            match time_rayon(args.bounds, args.upper_left, args.lower_right, args.rows_per_band, args.draw) {
                Ok(time) => println!("Time with rayon: {}ms", time),
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    }
}
