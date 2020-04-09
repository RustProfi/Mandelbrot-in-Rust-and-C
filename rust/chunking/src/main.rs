use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![0;10000]));
    let threads = 8;
    let chunk_size = (data.lock().unwrap().len() + 1) / 8;
    let mut threads = vec![];

    let range = 0..data.lock().unwrap().len();

    //würde man hier direkt in 0..data.len() gehen würde es ewig dauern weil der mutex nicht freigegeben wird
    for (i,offset) in range.clone().step_by(chunk_size).enumerate() {
        let chunk_length = if range.len() - offset > chunk_size {chunk_size} else {range.len() - offset};
        let ref_to_data = data.clone();
        threads.push(thread::spawn(move || {
                inside_thread(ref_to_data, offset, chunk_length, i)
            }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    //I would like to achieve an array filled with 1 here*/
    //println!("{:?}", data.lock().unwrap());

}
fn inside_thread(data: Arc<Mutex<Vec<u8>>>, offset: usize, chunk_size: usize, number: usize) {


    for xd in offset..offset + chunk_size {

        data.lock().unwrap()[xd] = number as u8;
        print!("{:?}", number);

    }
}
