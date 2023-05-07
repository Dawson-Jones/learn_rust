use std::{ops::{Add, Sub, Mul, Div, Rem, DerefMut}, fmt::{Display, write}};
use std::ops::Deref;

pub trait Draw {
    fn draw(&self);
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

// dyn 不能单独作为特征对象的定义，
// 原因是特征对象可以是任意实现了某个特征的类型，编译器在编译期不知道该类型的大小，不同的类型大小是不同的。
// 而 &dyn 和 Box<dyn> 在编译期都是已知大小，所以可以用作特征对象的定义。
// dyn 我的理解是, 告诉编译器实现这个 trait 的不是同一个类型, 动态的类型
// 官方回答: 直到运行时，才能确定需要调用什么方法。之前代码中的关键字 dyn 正是在强调这一“动态”的特点。
// 与泛型编译对立
// https://pic1.zhimg.com/80/v2-b771fe4cfc6ebd63d9aff42840eb8e67_1440w.jpg

/// trait object 的限制
/// - 方法的返回类型不能是 Self, 因为特征对象不知道具体类型是什么
/// - 方法没有任何泛型参数, 因为泛型参数的类型信息只能在编译时确定，而特征对象在运行时才能确定具体类型。

struct Screen {
    components: Vec<Box<dyn Draw>>
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 绘制SelectBox的代码
    }
}

impl<T> Draw for T 
    where T: Display
{
    fn draw(&self) {
        println!("{}", self)
    }
}

struct Point {
    x: i32,
    y: i32,
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point { }

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn main() {
    let x = 1.1;
    x.draw();

    // 特征定义中的特征约束 
    let p = Point {x: 3, y: 4};
    p.outline_print();

    // 孤儿原则的解决办法
    let mut t = MyVec(vec![1,2,3]);
    println!("t: {}", t);
    // 敲重点: MyVec 实现了 DrefMut, 所以 MyVec 可以使用 Vec 的 push 方法
    t.push(4);
    println!("t: {}", t);
}

struct MyVec<T>(Vec<T>);

impl<T: Display> Display for MyVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(vec_len: {}...)", self.0.len())
    }
}

impl<T> Deref for MyVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0       
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0       
    }
}