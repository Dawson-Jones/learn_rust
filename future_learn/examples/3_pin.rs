use std::{marker::PhantomPinned, pin::Pin};

/// Unpin 是一个特征，它表明一个类型可以随意被移动
/// TODO:

struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Self {
        let mut t = Test { 
            a: String::from(txt), 
            b: std::ptr::null(),
            // 这个标记可以让我们的类型自动实现特征 !Unpin: 未实现 Unpin, 即不可以被安全的移动
            _marker: PhantomPinned,
        };
        t.b = &t.a as *const String;

        t
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null());

        unsafe {
            &(*self.b)
        }
    }
}

fn main() {
    let mut test1 =Test::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    let mut test2 =Test::new("test2");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test2) };

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    std::mem::swap(test1.get_mut(), test2.get_mut());
    println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
}