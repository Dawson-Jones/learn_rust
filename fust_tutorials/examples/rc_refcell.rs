use std::{rc::Rc, cell::{Ref, RefCell}};


fn main() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));
    let s1 = s.clone();
    let s2 = Rc::clone(&s);

    s2
        .borrow_mut()
        .push_str("oh yeaah");
    
        println!("{:?}\n{:?}\n{:?}\n", s, s1, s2);
}