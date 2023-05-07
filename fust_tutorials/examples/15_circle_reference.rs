use std::{cell::RefCell, rc::{Rc, Weak}, borrow::Borrow};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}


fn main() {
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    println!("count a: {}", Rc::strong_count(&a));
    println!("a point to: {:?}", a.tail());

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));
    // 下面一行println!将导致循环引用
    // println!("a next item = {:?}", a.tail());


    // weak
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);

    drop(five);

    let strong_five = weak_five.upgrade();
    assert_eq!(strong_five, None);

    // 例子1
    struct Owner {
        name: String,
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }
    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    let gadget_owner = Rc::new(
        Owner {
            name: "Gadget Man".to_string(),
            gadgets: RefCell::new(Vec::new()),
        }
    );

    let gadget1 = Rc::new(Gadget{ id: 1, owner: gadget_owner.clone() });
    let gadget2 = Rc::new(Gadget{ id: 2, owner: gadget_owner.clone() });

    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget1));
    gadget_owner.gadgets.borrow_mut().push(Rc::downgrade(&gadget2));

    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }

    // 例子2
    let leaf = Rc::new(
        Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            clildren: RefCell::new(Vec::new()),
        }
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),    /* 1 */
        Rc::weak_count(&leaf),  /* 0 */
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            clildren: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),  /* 1 */
            Rc::weak_count(&branch),    /* 1 */
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); /* None */
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),    /* 1 */
        Rc::weak_count(&leaf),  /* 0 */
    );

}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    clildren: RefCell<Vec<Rc<Node>>>,
}


// unsafe
struct Tree {
    count: usize,
    root: *mut Nodee,
}

struct Nodee {
    value: i32,
    left: *mut Nodee,
    right: *mut Nodee,
    parent: *mut Nodee,
}

impl Tree {
    fn new() -> Self {
        Self {
            count: 0,
            root: std::ptr::null_mut(),
        }
    }

    
}