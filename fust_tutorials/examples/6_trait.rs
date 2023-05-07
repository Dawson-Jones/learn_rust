// 特征(trait)定义了一组可以被共享的行为，只要实现了trait，你就能使用这组行为。

use std::{iter::Sum, ops::Add, fmt::Display};

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// fn notify(iter1: &impl Summary, item2: &impl Summary) { }
// 保证 iter1 和 iter2 一定是同一种类型
fn notify<T: Summary>(iter1: &T, item2: &T) { }


// 要求 T 已经实现了加法操作, 这样子才能在后面进行 p1.x + p2.x
#[derive(Debug, Clone, Copy)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File { 
            name: String::from(name), 
            data: Vec::new(), 
            state: FileState::Closed 
        }
    }
}

fn main() {
    // 特征定义与实现的位置(孤儿规则)
    // 如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的

    // 加法运算
    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 3, y: 4};
    let p = p1 + p2;
    println!("{:?} + {:?} = {:?}", p1, p2, p);

    // Display
    let f6 = File::new("f6.txt");
    println!("{:?}", f6);   // File { name: "f6.txt", data: [], state: Closed }
    println!("{}", f6);     // <f6.txt (CLOSED)>
}