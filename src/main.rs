use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut n = m.lock().unwrap();
        *n = 6;
    }
    // let n = m;
    println!("m is {:?}", m);
}
