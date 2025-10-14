use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Muetx<T> is an abbrevation for Mutual Exclusion and, as in mutex allows only one thread to access some data at any given time. TO access data in Mutex thread first need to accuqire the lock this return MutexGurad<T>

    // let m = Mutex::new(5);

    // {
    //     let mut n = m.lock().unwrap();
    //     *n = 6;
    // }
    // // let n = m;
    // println!("m is {:?}", m);

    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result : {}", *counter.lock().unwrap());
}
