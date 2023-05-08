use std::{pin::Pin, task::{Context, Poll}};

/// Future, 通过调用 poll 来推进 Futre 的进一步执行
/// 若在当前 poll 中， Future 可以被完成，则会返回 Poll::Ready(result) ，
/// 反之则返回 Poll::Pending， 并且安排一个 wake 函数
/// 当未来 Future 准备好进一步执行时， 该函数会被调用，
/// 然后管理该 Future 的执行器(例如上一章节中的block_on函数)会再次调用 poll 方法，此时 Future 就可以继续执行了。
/// 
/// 猜测:
/// 也就是说, 当 poll 没有完成时, executor 向 future 注册了一个 wake 函数
/// 当 future 可以再次被执行时, 调用 wake 通知 executor, executor 将这个 future 放到就绪队列, 准备再次 poll
/// 如果没有 wake, executor 就需要轮询了
/// 
/// 

trait MyFuture {
    type Output;
    fn poll(
        // 具有固定的内存地址，意味着我们可以存储它的指针(如果内存地址可能会变动，那存储指针地址将毫无意义)
        self: Pin<&mut Self>,
        // `wake: fn()` 修改为 `cx: &mut Context<'_>`, 为了携带数据
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}


fn main() {
    // 见: exector
}