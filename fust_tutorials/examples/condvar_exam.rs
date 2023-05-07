use std::{sync::{Arc, Mutex, Condvar}, thread, time::Duration};

fn main() {
    let flag = Arc::new(Mutex::new(false));
    let condvar = Arc::new(Condvar::new());
    let cf = flag.clone();
    let ccv = condvar.clone();

    let handle = thread::spawn(move|| {
        let mut counter = 0;

        while counter < 3 {
            // 等待 cf 通知, 然后获取 mutex 的值?
            if *ccv.wait(cf.lock().unwrap()).unwrap() {
                *cf.lock().unwrap() = false;
                counter += 1;
                println!("inner counter: {}", counter);
            }
        }
    });
    let mut counter = 0;
    while counter < 3 {
        thread::sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        println!("outside counter: {}", counter);   
        condvar.notify_one();
    }

    handle.join().unwrap();
    println!("{:?}", flag);
}