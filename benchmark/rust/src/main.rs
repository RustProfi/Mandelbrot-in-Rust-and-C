use libc::c_int;
use libc::timespec;
use num::Complex;

fn main() {
    let mut x: f64 = 0.0;
    for _ in 0..1000 {
        x += time_complex();
    }
    x /= 1000.0;

    println!("{}", x);
}

fn time_complex() -> f64 {
    #[link(name = "c")]
    extern "C" {
        pub fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    }

    let mut start = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end = start.clone();

    let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
    let c: Complex<f64> = Complex {
        re: -0.32,
        im: 0.12,
    };

    unsafe {
        clock_gettime(4, &mut start);
    }

    for _ in 0..1_000_000 {
        z = z * z + c;
    }

    unsafe {
        clock_gettime(4, &mut end);
    }
    let mut vec = vec![];
    vec.push(z);

    //println!("{}", z);
    (end.tv_sec - start.tv_sec) as f64 * 1000.0 + (end.tv_nsec - start.tv_nsec) as f64 / 1_000_000.0
}
