// Rng stands for "random number generator".
use rand::Rng;
use std::thread;
use std::time::Duration;

//type GenericError = Box<dyn std::error::Error + Send + Sync>;

fn main() {
    const N: i32 = 3;

    // Start N threads and collect their handles.
    let mut handles = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 1..=N {
        // move is necessary for the closure to take ownership of i.
        handles.push(thread::spawn(move || {
            println!("thread {} started", i);
            // Generate a random number of milliseconds to sleep.
            let ms = rng.gen_range(500..5000);
            thread::sleep(Duration::from_millis(ms));
            println!("thread {} finished", i);
            (i, ms) // thread number and milliseconds slept
        }));
    }

    // Wait for all the threads to finish.
    for handle in handles {
        match handle.join() {
            Ok(result) => println!("result = {:?}", result),
            Err(e) => eprintln!("error: {:?}", e),
        }
    }

    println!("done");
}
