// 一系列不可被 CPU 上下文交换的机器指令，这些指令组合在一起就形成了原子操作。
// 在多核 CPU 下，当某个 CPU 核心开始运行原子操作时，会先暂停其它 CPU 内核对内存的操作，以保证原子操作不会被其它 CPU 内核所干扰。

use std::{sync::atomic::{AtomicU64, Ordering}, time::Instant, thread::{JoinHandle, self}, ops::Sub};

// 编译时常量
const N_TIMES: u64 = 10000000;
const N_THREADS: usize = 10;

// 运行时变量
static R: AtomicU64 = AtomicU64::new(0);

fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn main() {
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }
    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    println!("{:?}",Instant::now().sub(s));
}