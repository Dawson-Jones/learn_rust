// // 官方的一个全局变量的实现
use chrono::Utc;
use std::{sync::{Mutex, Once}, borrow::BorrowMut};

static mut STD_ONCE_COUNTER: Option<Mutex<String>> = None;
static INIT: Once = Once::new();

fn global_string<'a>() -> &'a Mutex<String> {
    INIT.call_once(|| {
        unsafe {
            *STD_ONCE_COUNTER.borrow_mut() = Some(Mutex::new(Utc::now().to_string()));
        }
    });

    unsafe {
        STD_ONCE_COUNTER.as_ref().unwrap()
    }
}

fn main() {
    // let now = global_string().lock().unwrap();
    println!("Global string is {}", *global_string().lock().unwrap());

    *global_string().lock().unwrap() = Utc::now().to_string();

    println!("Global string is {}", *global_string().lock().unwrap());
}


// pub fn main() {
//     println!("Global string is {}", *global_string().lock().unwrap());
//     *global_string().lock().unwrap() = Utc::now().to_string();
//     println!("Global string is {}", *global_string().lock().unwrap());
// }