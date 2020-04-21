use std::sync::Arc;
use std::sync::Mutex;
use std::process::exit;
use std::thread;

fn main() {
    //An array nested inside a Mutex nested inside an Arc
    let arr = Arc::new(Mutex::new(vec![0; 5]));

    //create a new reference to arr
    //and increment the reference count
    let arr_ref = arr.clone();
    //Closure takes Ownership of arr_ref
    match thread::spawn(move || fillvec(arr_ref)).join() {
        Ok(_) => {},
        Err(_) => {
            eprintln!("Thread failed");
            exit(1);
        }
    }
    println!("{:?}", arr.lock().unwrap());
}

fn fillvec(arr: Arc<Mutex<Vec<i32>>>) {
    //Acquire the lock assuming the Ok case
    let mut guard = arr.lock().unwrap();
    for i in 0..guard.len() {
        guard[i] = (i + 1) as i32;
    }
} //guard gets dropped here and lock is released
