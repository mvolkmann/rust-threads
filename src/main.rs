use rand::Rng; // stands for "random number generator".
use std::sync::mpsc; // stands for "multiple producer, single consumer"
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    const N: i32 = 3;

    // Start N threads and collect their handles.
    let mut handles = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 1..=N {
        let ms = rng.gen_range(500..5000);
        // The "move" keyword below is necessary for
        // the closure to take ownership of i and ms.
        let tx = mpsc::Sender::clone(&tx);
        handles.push(thread::spawn(
            move || -> Result<(i32, u64), mpsc::SendError<_>> {
                println!("thread {} started", i);
                // Generate a random number of milliseconds to sleep.
                thread::sleep(Duration::from_millis(ms));
                let msg = format!("tx from thread {}!", i);
                tx.send(msg)?;
                println!("thread {} finished", i);
                Ok((i, ms)) // tuple of thread number and milliseconds slept
            },
        ));
    }
    // Need to drop the original sender (tx) so the receiver (rx) so
    // the loop below can exit after the last tx clone goes out of scope.
    drop(tx);

    // Listen for channel messages in a different thread.
    // This is a good way to enable processing the results from each thread
    // in the order in which they complete
    // rather than the order in which they were started.
    for msg in rx {
        println!("received {}", msg);
    }

    // Wait for all the threads to finish.
    // Results will be processed in the order the threads were created,
    // not in the order in which the threads complete.
    for handle in handles {
        match handle.join() {
            Ok(result) => println!("result = {:?}", result.unwrap()),
            Err(e) => eprintln!("error: {:?}", e),
        }
    }

    println!("done");
}
