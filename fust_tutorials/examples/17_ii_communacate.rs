use std::sync::{mpsc, Mutex, Arc};
use std::{thread, vec};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    // 需要所有的发送者都被drop掉后，接收者rx才会收到错误，进而跳出for循环，最终结束主线程
    drop(tx);   // 如果这里不 drop, rx 不会关闭
    thread::spawn(move || {
        tx1.send(String::from("hi from raw tx")).unwrap();
    });

    thread::spawn(move || {
        tx2.send(String::from("hi from cloned tx")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // 使用一个带缓冲值的同步通道来避免内存过大的问题
    // 异步通道上限取决于内存的大小
    let (tx, rx) = mpsc::sync_channel(1);
    let t1 = thread::spawn( move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
        tx.send(1).unwrap();
        println!("再次发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");
    for item in rx {
        println!("receive {}", item);
    }

    t1.join().unwrap();

    // 消息传递 or 共享内存
    // 消息传递类似一个单所有权的系统：一个值同时只能有一个所有者，如果另一个线程需要该值的所有权，需要将所有权通过消息传递进行转移。
    // 共享内存类似于一个多所有权的系统：多个线程可以同时访问同一个值。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let t = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;  // Mutex 实现了内部可变性!
            // *counter.lock().unwrap() += 1;
        });
        handles.push(t);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
    // Send特征可以确保数据在线程中安全的传输
}
