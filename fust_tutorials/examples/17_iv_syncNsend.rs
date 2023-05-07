use std::{thread, sync::Arc};

use futures::lock::Mutex;

/// 实现Send的类型可以在线程间安全的传递其所有权
/// 实现Sync的类型可以在线程间安全的共享(通过引用)
/// 潜在的依赖: 
/// 一个类型要在线程间安全的共享的，指向它的引用必须能在线程间传递。
/// 因为如果引用都不能被传递，我们就无法在多个线程间使用引用去访问同一个数据了。
/// 也就是说, 若一个类型的引用是 Send, 那么该类型是 Sync 的
/// Rust 中绝大多数类型都实现了Send和Sync, 除了
/// - 裸指针
/// - UnsafeCell不是Sync，因此Cell和RefCell也不是
/// - Rc两者都没实现(因为内部的引用计数器不是线程安全的)

#[derive(Debug)]
struct MyBox(*mut u8);
// Send和Sync是unsafe特征，实现时需要用unsafe代码块包裹。
unsafe impl Send for MyBox { }
unsafe impl Sync for MyBox { }

fn main() {
    // 为裸指针实现 Send(线程间安全传递所有权)
    let p = MyBox(5 as *mut u8);
    let t = thread::spawn(move || {
        println!("{:?}", p);
    });

    t.join().unwrap();

    // 为裸指针实现 Sync, 通过引用线程间共享
    let p = &MyBox(5 as *mut u8); // MyBox 实例是一个匿名对象，只有当 p 存在时才存在。
    // let p = MyBox(5 as *mut u8); // 这样定义有生命周期的问题
    let v = Arc::new(p);   // 共享时最好使用 mutex
    let t = thread::spawn( move || {
        println!("P: {:?}", **v);
    });
    t.join().unwrap();
}