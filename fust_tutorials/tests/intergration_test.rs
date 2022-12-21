use fust_tutorials;


#[test]
fn it_adds_two() {
    assert_eq!(4, fust_tutorials::add_two(2))
}


struct self_ref {
    value: String,
    pointer: *const String,
}


impl self_ref {
    fn new(txt: &str) -> Self {
        self_ref { value: txt.to_string(), pointer: std::ptr::null() }
    }

    fn init(&mut self) {
        let p: *const String = &self.value;
        self.pointer = p;
    }

    fn get_value(&self) -> &str {
        &self.value
    }

    fn get_pointer(&self) -> &String {
        unsafe {
          &*(self.pointer)
        }
    }
}

#[test]
fn test_selfref() {
    let mut t1 = self_ref::new("test1");
    t1.init();
    let mut t2 = self_ref::new("test2");
    t2.init();

    println!("t1  value: {}, pointer: {}", t1.get_value(), t1.get_pointer());
    println!("t2  value: {}, pointer: {}", t2.get_value(), t2.get_pointer());
    assert_eq!(t1.get_value(), "test1");
    assert_eq!(t1.get_pointer(), "test1");
    assert_eq!(t2.get_value(), "test2");
    assert_eq!(t2.get_pointer(), "test2");

    std::mem::swap(&mut t1, &mut t2);

    assert_eq!(t1.get_value(), "test2");
    assert_eq!(t1.get_pointer(), "test1");
    assert_eq!(t2.get_value(), "test1");
    assert_eq!(t2.get_pointer(), "test2");
}