use std::{ptr::NonNull, marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct SelfRef {
    value: String,
    pointer_to_value: *const String,
}

impl SelfRef {
    fn new(txt: &str) -> Self {
        let mut slf = SelfRef { 
            value: txt.to_string(), 
            pointer_to_value: std::ptr::null() 
        };
        let slf_ref: *const String = &slf.value;
        slf.pointer_to_value = slf_ref;

        slf
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn pointer_to_value(&self) -> &String {
        assert!(!self.pointer_to_value.is_null());

        // 解引用裸指针是不安全的
        // 裸指针可能是 null, 悬垂的, 非对齐的
        // they can violate aliasing rules and cause data races: all of these are undefined behavior
        unsafe { &*self.pointer_to_value }
    }
}

fn main() {
    // #[derive(Debug)]
    // struct WhatAboutThis<'a> {
    //     name: String,
    //     nickname: &'a str,
    // }
    // let s = "Annabelle".to_string();
    // let mut tricky = WhatAboutThis {
    //     name: s,    // 所有权移动
    //     nickname: &s,   // 借用
    // };  // 移动和借用同时发生
    // println!("{:?}", tricky);

    let t = SelfRef::new("hello");
    println!("{}, {}, {:p}, {:p}", 
        t.value(), t.pointer_to_value(), 
        t.pointer_to_value(), t.pointer_to_value);
}

// Pin: 固定住一个值，防止该值在内存中被移动
struct Unmovable {
    data: String,
    slice: NonNull<String>,
    _pin: PhantomPinned,
}

impl Unmovable {
    // 为确保函数返回的数据不会内存不会被改变, 将它放在堆上
    fn new(data: String) -> Pin<Box<Self>> {
        let mut res = Box::pin(Unmovable {
            data,
            slice: NonNull::dangling(),
            _pin: PhantomPinned,
        });

        let slice = NonNull::from(&res.data);

        unsafe {
            let mut_ref = Pin::as_mut(&mut res);
            Pin::get_unchecked_mut(mut_ref).slice = NonNull::from(slice);
        }
        res
    }
}