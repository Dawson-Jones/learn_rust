use std::{
    cell::{Ref, RefCell},
    sync::{Arc, Barrier, Condvar, Mutex, Once},
    thread,
    time::Duration,
};

// 线程局部变量
thread_local! {static FOO: RefCell<u32> = RefCell::new(1)}

fn main() {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // barrier: 让多个线程都执行到某个点后，才继续一起往后执行
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
            b.wait(); // 用 b.wait 等所有线程都执行完, 才继续往下执行, 有点像信号量
            println!("after wait");
        }))
    }

    for handle in handles {
        handle.join().unwrap()
    }
    // 线程执行完毕, 变量依然是主线程修改的2
    FOO.with(|f| assert_eq!(*f.borrow(), 2));

    // 条件变量Condition Variables, 让线程挂起，直到某个条件发生后再继续执行
    // 看起来有点像信号
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed: {}", started);

    // 只能被调用一次的函数
    let t1 = thread::spawn(move|| {
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        })
    });
    let t2 = thread::spawn(move|| {
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        })
    });
    t1.join().unwrap();
    t2.join().unwrap();

    println!("VAL: {}", unsafe { VAL });
}


static mut VAL: usize = 0;
static INIT: Once = Once::new();