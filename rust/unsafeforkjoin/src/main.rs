use std::sync::Arc;
use std::thread;
use std::cell::UnsafeCell;
use std::ptr;
use std::mem;

fn main() {
    unsafe {xd();}
}

struct Wrapper<T>(UnsafeCell<T>);
unsafe impl<T> Send for Wrapper<T> {}
unsafe impl<T> Sync for Wrapper<T> {}

unsafe fn xd() {
    let mut v = vec![0 as u8;10];
    // Prevent running `v`'s destructor so we are in complete control
// of the allocation.
    let mut v = mem::ManuallyDrop::new(v);
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
unsafe fn threadlol(x: Arc<Wrapper<*mut u8>>, i: isize) {
        let ptrxd = *x.0.get();
        ptr::write(ptrxd.offset(i), i as u8);
}
