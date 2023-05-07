use std::ops::Deref;

/// 智能指针往往是基于结构体实现，与自定义的结构体最大的区别在于它实现了 `Deref` 和 `Drop` 特征
/// - `Deref` 可以让智能指针像引用那样工作，这样就可以写出同时支持智能指针和引用的代码，例如 *T
/// - `Drop` 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作
///     一个车类型无法同时实现 Copy 和 Drop 特征



struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0       
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

fn main() {
    let b = MyBox::new(12);
    println!("12 + 1 = {}", *b + 1);
}