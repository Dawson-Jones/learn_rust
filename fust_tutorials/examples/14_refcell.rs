use std::{cell::{Cell, RefCell}, rc::Rc};

/// Cell 和 RefCell 
/// 用于内部可变性，可以在拥有不可变引用时修改目标数据


// 定义在外部库中的特征
pub trait Messenger {
    fn send(&self, msg: String);
}

// 需要缓存发送的消息
struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl MsgQueue {
    fn new() -> Self {
        MsgQueue {
            msg_cache: RefCell::new(Vec::new())
        }
    }
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}

fn main() {
    let mq = MsgQueue::new();

    mq.send(String::from("hello, world"));

    // Rc + RefCell
    // 前者让数据有多个所有者, 后者实现数据可变性
    let s = Rc::new(RefCell::new(String::from("我很善变, 还拥有多个主人")));

    let s1 = s.clone();
    let s2 = s.clone();

    s2.borrow_mut().push_str(", oh yeah");
    println!("{:?}\n{:?}\n{:?}", s, s1, s2);

    //
}


/// - Cell::from_mut，该方法将 &mut T 转为 &Cell<T>
/// - Cell::as_slice_of_cells，该方法将 &Cell<[T]> 转为 &[Cell<T>]
fn retain_even(nums: &mut Vec<i32>) {
    let slice = Cell::from_mut(&mut nums[..])
        .as_slice_of_cells();

    let mut i = 0;
    for num in slice.iter().filter(|num| num.get() % 2 == 0) {
        slice[i].set(num.get());
        i += 1;
    }

    nums.truncate(i);
}