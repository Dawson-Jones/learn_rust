// fn x() {
//     let mut v = Vec::<(String, &str)>::new();

//     loop {
//         let string = String::from("hi");
//         let str = string.trim();

//         v.push((string, str));
//         while v.len() > 5 {
//             v.remove(0);
//         }
//     }
// }


#[derive(Debug)]
struct SelfRef {
    value: String,
    pointer_to_value: *mut String,
}

impl SelfRef {
    fn new(txt: String) -> Self {
        SelfRef {
            value: txt,
            pointer_to_value: std::ptr::null_mut(),
        }
    }

    fn init(&mut self) {
        self.pointer_to_value = &mut self.value;
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn pointer_to_value(&self) -> &String {
        assert!(!self.pointer_to_value.is_null(),
            "Test::b called without Test::init being called first");
        unsafe { &*(self.pointer_to_value) }
    }
}

fn main() {
    let txt = String::from("hello");
    println!("txt: {:p}", &txt);
    let mut t = SelfRef::new(txt);
    t.init();
    // 打印值和指针地址
    println!("{}, {:p}", t.value(), t.pointer_to_value());

    t.value.push_str(", world");
    unsafe {
        (*t.pointer_to_value).push_str("!");
    }

    println!("{}, {:p}", t.value(), t.pointer_to_value());
}
