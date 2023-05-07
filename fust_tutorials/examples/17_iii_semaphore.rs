use std::{sync::Arc, thread, time::Duration};
use tokio::sync::Semaphore;




#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(2));
    let mut handles = vec![];

    for i in 0..5 {
        let semaphore = semaphore.clone();
        handles.push(tokio::spawn(async move {
            let permit = semaphore.acquire_owned().await.unwrap(); 
            //
            println!("thread {} process", i);
            thread::sleep(Duration::from_millis(1000));
            drop(permit);
        }))
    }

    for handle in handles {
        handle.await.unwrap();
    }
}