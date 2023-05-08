use tokio::task::yield_now;

/// tokio
/// - 一个异步代码的多线程运行时(相当于 go 语法)
/// - 标准库的异步版本


#[tokio::main]
async fn main() {
    // 通过 spawn 创建一个 task
    // task 是 executor 的调度单元
    // 一个任务 64 Bytes 的内存分配
    let handle = tokio::spawn(async {
        10086
    });
    let out = handle.await.unwrap();
    println!("out: {}", out);

    yield_now()
}