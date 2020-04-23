extern crate libc;
#[cfg(test)]
extern crate mandelbrot;
extern crate num;
use libc::timespec;
use mandelbrot::mandel::pixel_to_point;
use mandelbrot::time::MyTimestamp;
use num::Complex;

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

#[test]
fn test_compute_time_millis() {
    let x1 = MyTimestamp {
        ts: timespec {
            tv_sec: 3,
            tv_nsec: 0_000_000_000,
        },
    };

    let x2 = MyTimestamp {
        ts: timespec {
            tv_sec: 4,
            tv_nsec: 0_000_000_000,
        },
    };

    let x3 = MyTimestamp {
        ts: timespec {
            tv_sec: 4,
            tv_nsec: 0_500_000_000,
        },
    };

    let x4 = MyTimestamp {
        ts: timespec {
            tv_sec: 6,
            tv_nsec: 0_200_000_000,
        },
    };

    assert_eq!(x1.compute_time_millis(x2.clone()), 1000 as f64);
    assert_eq!(x1.compute_time_millis(x3.clone()), 1500 as f64);
    assert_eq!(x2.compute_time_millis(x3.clone()), 500 as f64);
    assert_eq!(x3.compute_time_millis(x4), 1700 as f64);
}
