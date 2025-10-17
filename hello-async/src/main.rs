use std::time::Duration;

fn main() {
    // using trpl::spawn_task to understand something like thread::spwan 

    // trpl::run(async {
    //     let handle = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi from number {i} from the first task");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("hi from number {i} from the second task");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }

    //     handle.await.unwrap();
    // })

    // using trpl::join instead of .join()

    trpl::run (async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number from {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number from {i} from the second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    })
}

