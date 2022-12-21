use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};


fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(rt.spawn(my_bg_task(i)));
    }

    // std::thread::sleep(Duration::from_millis(750));
    // println!("Finished time-consuming task");

    for handle in handles {
        rt.block_on(handle).unwrap();
    }
}

async fn my_bg_task(i: u64) {
    let millis = 1000 - 50 * i;
    println!("Task {} sleeping for {}ms", i, millis);
    sleep(Duration::from_millis(millis)).await;
    println!("Task {} stopping", i);
}