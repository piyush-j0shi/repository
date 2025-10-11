use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let (tx, rx) = mpsc::channel();
    let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // there are two methods for reciever one is `recv` and another one is `try_recv` and the diffrence is:

    // think of this like if messages sent by transmiter are being stored in queue and recv will wait for transmiter that will the transmitter send a message or an error and if the transmitter sends a message it will transmit the message to the reciever and if transmitter sends an error it will transmit the error and for this process it will wait or hold the main thread execution.

    // And the `try_recv` will not hold the main thread for execution if there is a message in queue it will transmit the message othervise it will transmit the error doesn't matter the status of transmitter if there is a message it will transmit and if there is not then it will not block the main thread doesn't matter if the transmitter is active or not.

    // let recieved = rx.recv().unwrap();
    // println!("got : {recieved}");

    for recieved in rx {
        println!("got {recieved}");
    }
}
