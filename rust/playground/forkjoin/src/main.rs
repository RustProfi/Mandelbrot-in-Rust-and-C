use std::sync::Arc;
use std::thread;
use std::cell::UnsafeCell;
use std::ptr;

fn main() {
    let x: Arc<[u8]> = vec![1, 2, 3, 4, 5].into();
    //Durch das Klonen wird nur die Referenz geklont!
    run_thread(x.clone());
    //Der Elternprozess sowie der Kindprozess halten eine gültige
    //Refernz auf x.
    println!("{:?}", x);
    unsafe {xd();}
}

fn run_thread(x: Arc<[u8]>) {
    let thread = thread::spawn(move || println!("{:?}", x));
    thread.join().unwrap();
}

struct Wrapper<T>(UnsafeCell<T>);
unsafe impl<T> Send for Wrapper<T> {}
unsafe impl<T> Sync for Wrapper<T> {}

unsafe fn xd() {
    let mut v = vec![0 as u8;10];
    //let mut v = mem::ManuallyDrop::new(v);
    let p: *mut u8 = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();
    let myv = Arc::new(Wrapper(UnsafeCell::new(p)));




    for i in 0..10 {
        let ptr = myv.clone();
        thread::spawn(move || threadlol(ptr, i)).join();
    }

    let vec = Vec::from_raw_parts(*myv.0.get(), len, cap);
    println!("{:?}", vec);


}
fn threadlol(x: Arc<Wrapper<*mut u8>>, i: isize) {
    unsafe {
        let ptrxd = *x.0.get();
        ptr::write(ptrxd.offset(i), i as u8);

    }

}
