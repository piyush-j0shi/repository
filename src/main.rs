// use std::rc::Rc;
use std::sync::{Arc, Mutex};
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

    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // We can not use Rc<T> here because Rc<T> keeps the refrence counts but it does not have concurrency primitives so It can be interrupted by another thread or because of multiple thread

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);

    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("result : {}", *counter.lock().unwrap());

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
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
