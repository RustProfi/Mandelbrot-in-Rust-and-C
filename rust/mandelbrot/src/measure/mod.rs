use crate::wrayon::time_with_rayon;
use crate::wcrossbeam::time_with_crossbeam;
use crate::wscopedthreadpool::time_with_scoped_threadpool;
use crate::wthreadsunsafe::time_threads_unsafe;
use crate::customerror::CustomError;
use crate::wthreads::time_threads;
use num::Complex;
use std::fs::File;
use std::io::prelude::*;

pub fn measure_workload_threads(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    let mut file = File::create("rust_threads_performance.txt")?;

    for thread_count in 4..=60 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_threads(bounds, upper_left, lower_right, thread_count, false)?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", thread_count, time))?;
    }
    Ok(())
}

pub fn measure_workload_threads_unsafe(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    let mut file = File::create("rust_threads_unsafe_performance.txt")?;

    for thread_count in 4..=60 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_threads_unsafe(bounds, upper_left, lower_right, thread_count, false)?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", thread_count, time))?;
    }
    Ok(())
}

pub fn measure_workload_crossbeam(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    let mut file = File::create("rust_crossbeam_performance.txt")?;

    for thread_count in 4..=60 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_with_crossbeam(bounds, upper_left, lower_right, thread_count, false)?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", thread_count, time))?;
    }
    Ok(())
}

pub fn measure_workload_scoped_threadpool(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    number_of_threads: u32
) -> Result<(), CustomError> {
    let mut file = File::create("rust_scoped_threadpool_performance.txt")?;

    for rows_per_band in 1..=60 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_with_scoped_threadpool(bounds, upper_left, lower_right, rows_per_band, number_of_threads, false)?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", rows_per_band, time))?;
    }
    Ok(())
}

pub fn measure_workload_rayon(
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<(), CustomError> {
    let mut file = File::create("rust_rayon_performance.txt")?;

    for rows_per_band in 1..=60 {
        let mut time: f64 = 0.0;
        for _ in 0..20 {
            time += time_with_rayon(bounds, upper_left, lower_right, rows_per_band, false)?;
        }
        time /= 20.0;
        file.write_fmt(format_args!("{},{}\n", rows_per_band, time))?;
    }
    Ok(())
}
