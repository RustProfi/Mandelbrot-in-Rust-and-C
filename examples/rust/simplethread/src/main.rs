use std::process::exit;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    //An array nested inside a Mutex nested inside an Arc
    let arr = Arc::new(Mutex::new(vec![0; 5]));

    //create a new reference to arr
    //and increment the reference count by 1
    let arr_ref = arr.clone();
    //Closure takes Ownership of arr_ref due to move
    if thread::spawn(move || fillvec(arr_ref)).join().is_err() {
        eprintln!("Thread paniced");
        exit(1);
    }
    println!("{:?}", arr.lock().unwrap());
} //arr gets dropped and freed finally here because no other

fn fillvec(arr: Arc<Mutex<Vec<i32>>>) {
    //Acquire the lock assuming the Ok case
    let mut guard = arr.lock().unwrap();
    for i in 0..guard.len() {
        guard[i] = (i + 1) as i32;
    }
} //guard gets dropped here and lock is released
  //arr gets dropped and reference count is decremented by 1
