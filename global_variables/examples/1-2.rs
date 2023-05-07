// single thread

use std::{cell::RefCell, borrow::Borrow};

use chrono::Utc;

// 需要使用闭包来访问
// 可以执行任意的初始化代码, 这些代码在第一次访问该值的时候运行
thread_local! {static GLOBAL_DATA: String = Utc::now().to_string()}

// 内部可变性
thread_local! {static GLOBAL_INTERIOR_MUTABILITY_DATA: RefCell<String> = RefCell::new(Utc::now().to_string())}

fn main() {
    // 使用
    GLOBAL_DATA.with(|text| {
        println!("hahah: {}", *text);
    });

    // 第一次使用, 初始化
    GLOBAL_INTERIOR_MUTABILITY_DATA.with(|text| {
        let n = &*text.borrow();
        println!("Global string is {}", n)
    });

    // 改变
    GLOBAL_INTERIOR_MUTABILITY_DATA.with(|text| {
        let mut p = text.borrow_mut();
        *p = Utc::now().to_string();
    });

    // 再次读取
    GLOBAL_INTERIOR_MUTABILITY_DATA.with(|text| {
        println!("Global string is {}", *text.borrow());
    });
}