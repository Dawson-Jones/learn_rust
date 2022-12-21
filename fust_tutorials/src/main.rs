use std::fmt::{Display, Debug};
use std::io::{self, ErrorKind, Write, Read};


fn variable_test() {
    // 变量默认不可变
    let x = 5;
    // x = 6;   // 非法
    let x = "hello, world";  // 第一个变量 x, 被隐藏了
    print!("x is {}\n", x);
}

fn typeofdata() {
    // 显式注明变量类型, 不能智能判断
    let guess: u32 = "42".parse().expect("not a number");
    print!("guess is {}\n", guess);
}

fn arra_y() {
    let a = [3; 5];
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index extered was not a number");

    let element = a[index];
    println!("value is {}", element)
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn func() {

    let x = add(4, 5);
    println!("4 add 5 is {}", x);
}

fn judge(x: i32) {
    let number = if x > 0 {1} else {0};
    println!("number is {}", number);
}

fn cycle() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2;  // break 有返回值
        }
    };

    println!("result is {}", result);
}

fn label() {
    let mut i = 1;
    'counting_up: loop {    // break `counting_up 从该层循环退出
        let mut j =  1;
        loop {
            if i >= 2 {
                break 'counting_up;
            }

            if j > 9 {
                break;
            }

            println!("{} * {} = {}", i, j, i * j);
            j+=1;

        }

        i += 1;
    }
}

fn use_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
}

fn for_in() {
    let arr = [3;4];

    for (i, e) in arr.iter().enumerate() {
        println!("the {}th element is {}", i, e);
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}

fn get_ownership() -> String {
    String::from("yours")
}

fn take_and_back(s: String) -> String {
    s
}

fn ownership() {
    let s1 = get_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_back(s2);

    println!("s1: {}", s1);
    // println!("s2: {}", s2);  // 所有权被转移
    println!("s3: {}", s3);
}

fn reference() {
    let mut s = String::from("hello, ");
    change(&mut s);
    println!("now s is: {}", s);

    let r1 = &mut s;
	// s.push_str("~~");         // 这样也会报错: 不能同时有两个可变引用
    r1.push_str("!");
    println!("r1 is {}", r1);

    s.push_str("~~");
    // println!("r1 is {}", r1);    // r1 被使用,报错
    println!("s is {}", s);   // r1 不再使用, 没问题
}

fn change(s: &mut String) {
    s.push_str("world");
}


fn slice() {
    slice_str();

    slice_num();
}

fn slice_num() {
    let a = [1,2,3,4,5];
    let slice: &[i32] = &a[1..3];

    assert_eq!(slice, &[2,3]);
}

fn slice_str() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world: &str = &s[6..];
    println!("{}, {}", hello, world);

    let first = first_word(&s);
    
    println!("first word is `{}`", first);

}

fn first_word(s: &str) -> &str {    // s &String 隐式转换为 &str
    let bytes = s.as_bytes();

    // 这里的 c 为什么要加 &
    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[derive(Debug)]
// or manually `impl Debug for User
struct User {
    active: bool,
    name: String,
    email: String,
}

fn structure() {
    let mut user = User {
        active: true,
        name: String::from("dawson"),
        email: String::from("957360688@qq.com"),    // 不能少任何一个字段, 奇怪
    };
    user.email = String::from("jeedq@qq.com");

    // 如果后面不用了, 可以把 user.email 转移, 如果用,就不能转移
    // let anthor_user = build_user(user.name, user.email);
    // println!("email: {}", user.email)   // 这里使用了, 则不能转移

    println!("email: {}", user.email);   // 使用后不再使用, 可以转移
    let two = build_user(user.name, user.active);

    let three = User {
        name: String::from("nera"),
        ..user  // 展开 user, active 是复制类型, email 还没有被转移
    };

    println!("user: {:?}", two)
}

fn build_user(name: String, active: bool) -> User {
    User { active, name, email:String::from("xxx@qq.com") }  // 变量名和结构体名一样, 简化
}

fn tuple_struct() {
    struct Color(i32, i32, i32);

    let klein_blue = Color(0, 47, 167);

    println!("klein bule is R: {}, G: {}, B: {}", klein_blue.0, klein_blue.1, klein_blue.2);
}


fn method() {
    let rec = Rectangle {
        width: 4,
        height: 5
    };

    println!("area is {}", rec.area());

    println!("new a square.");
    let squ = Rectangle::square(5);
    println!("square's ares is {}", squ.area())
}

fn struct_simple() {
    let rec = Rectangle {
        width: 3,
        height: 4,
    };

    println!("area: {}", area(rec));

    // 这里也是不可用的
    // 因为定义的 Rectangle 这个结构体没有实现 Copy
    // move occurs because `rec` has type `Rectangle`, 
    // which does not implement the `Copy` trait
    // println!("area: {}", rec.height * rec.width);

    let x = 3;
    let y = back(x);

    // 这里则是可用的
    println!("x: {}", x);
}

fn area(rec: Rectangle) -> u32 {
    rec.width * rec.height
}

fn back(x: u32) -> u32 {
    x
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &str {
        let x = match self {
                Message::Quit => "quit",
                Message::Move{x, y} => "move",
                Message::Write(str) => str,
                _ => "other"
            };

            x
    }
}

fn enumerate() {
    let m = Message::Write(String::from("hello"));
    let str = m.call();
    println!("message call is {}", str);
}

fn if_let() {
    let some_num = Some(2);

    let x = if let Some(i) = some_num {
        i
    } else {    // 相当于 match 中的 _
        0
    };
}

fn while_let() {
    let mut stack = vec![1,2,3];

    while let Some(top) = stack.pop() {
        println!("while let: {}", top)
    }
}

fn module() {
}

fn vector() {
    let mut v = vec![1,2,3];

    let first = &v[0];
    v.push(4);

    // push 改变了 v，所以 first 有可能失效。
    // println!("first element is {}", first);

    let last = v.pop();
    match last {
        Some(i) => println!("last is {}", i),
        None => println!("vector empty"),
    };

    for i in &mut v {
        *i += 50;
    }

    println!("vector now {:?}", v);
}

fn string() {
    let mut s1 = String::from("foo");
    let temp = "bar";

    s1.push_str(temp);
    // 说明 temp 可复制
    println!("temp can use: {}", temp);

    let mut s2 = String::from("lo");
    s2.push('l');

    // + 相当于
    // fn add(self, s: &str) -> String
    // 所以 s1 也不再有效
    let s3 = s1 + &s2;  
    println!("s3: {}", s3);

    let s4 = format!("{}-{}", s2, s3);
    println!("s4: {}", s4);

    let hello = "Здравствуйте".to_string();
    let ans = &hello[0..2]; // 慎用，用可能 panic

    for c in hello.chars() {
        print!("{} ", c);
    }
    println!();

    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();
}


use std::collections::HashMap;
fn 哈希() {
    let mut scores: HashMap<String, usize> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // insert 相同的值，会被覆盖
    scores.entry("Yellow".to_string()).or_insert(50);

    let value = scores.get("Blue");
    for (key, value) in &scores {
        println!("{}: {}", key, value); // 无序打印 unordered map
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut score: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();


    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map: {:?}", map);
}


use std::fs::{File, OpenOptions};
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
// use std::thread;
use std::time::Duration;

use fust_tutorials::{Tweet, Summary, Rectangle, Messager, Post};
fn error_handle() {
    let file = "hello.txt";

    let mut f = match OpenOptions::new().read(true).write(true).open(file) {
    // let mut f = match File::open(file) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file) {
                Ok(fc) => fc,
                Err(e) => panic!("creat file failed: {}", e),
            }

            other_error => panic!("open file: {}", other_error),
        }
    };

    let text = "hello, world".as_bytes();
    match f.write(text) {
        Ok(num) => println!("write {} bytes to {}", num, file),
        Err(e) => panic!("error write: {}", e)
    };
}

fn error_handle_with_wrap_helper(file: &str) -> Result<String, io::Error> { // 枚举是一个范型, 范型用<>

    // 闭包
    let mut f = File::open("file").unwrap_or_else(|error | {
        if error.kind() == ErrorKind::NotFound {
            File::create(file).expect("Failed to create file")
        } else {
            panic!("Problel opening file: {:?}", error);
        }
    });


    let mut s = String::new();
    // ------------------------------
    // match f.read_to_string(&mut s) {
    //     Ok(num) => {
    //         println!("read {} bytes to buf", num);
    //         Result::Ok(s)
    //     },
    //     Err(e) => Err(e)
    // }
    // 可简写为
    let num = f.read_to_string(&mut s)?;
    // 1. 如果 Result 的值是 Ok，返回 Ok 中的值,程序继续执行。
    // 2. 如果值是 Err，Err 中的值将作为整个函数的返回值，
    // 就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。
    // 上面的 File::open 也可以用?来简化

    println!("read {} bytes to buf", num);
    Ok(s)
    // -----------------------------

}

fn error_handle_with_wrap() {
    let file = "hello.txt";

    let mut s = error_handle_with_wrap_helper(file).unwrap();
    s.push_str("...");
    println!("the {}'s content is:", file);
    println!("{}", s);
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut ret: T = list[0];

    for &ele in list {
        if ele > ret {
            ret = ele;
        }
    }

    ret
}

fn generic_type() {
    let numbers = vec![4,6,9,2,4,7];

    let ret = largest(&numbers);
    println!("the largest is {}", ret);
}


fn impl_trait() {
    let tweet = Tweet {
        author: "dawson".to_string(),
        content: "dawson is a good boy".to_string(),
        reply_number: 255,
        can_retweet: false
    };

    notify(&tweet);
}

// trait 作为参数
// fn notify<T: Summary>(tweet: &T) {
fn notify(tweet: &impl Summary) {
    println!("{}", tweet.summarize());
}

fn some_func<T, U>(t: &T, u: &U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}

// fn lifetime() {
//     let ret;
//     let x = String::from("xxxxxx");
//     {
//         let y = String::from("yyyyyyyyyyyy");
//         // 这里要保证 ret 的引用释放时机在前 (x 和 y 释放早的那个)
//         ret = longest(&x, &y);
//     }

//     println!("the longer str is: {}", ret);
// }

//意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
struct Excerpt<'a> {
    part: &'a str,
}

fn lifetime() {
    let novel = String::from("Call me Dawson. some years ago...");
    let first_sentence = novel.split('.').next().expect("Cound not find a '.'");
    let excerpt = Excerpt {
        part: first_sentence
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn compare<'a, T>(x: &'a str, y: &'a str, conn: T) -> &'a str 
    where T: Display
{
    println!("display: {}", conn);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn simple() {
    let x = String::from("hello, world!");
    let y = String::from("Dawson Jones");
    let conn = "I'm the king of the world";

    println!("the longer: {}", compare(&x, &y, conn));
}

// struct Cacher {
//     // `impl Trait` only allowed in function and inherent method return types, not in type
//     calculation: impl Fn(u32) -> u32,
// }

use std::hash::Hash;
struct Cacher<T, U> 
    where T: Fn(U) -> U, 
          U: Hash + Eq,
{
    calculation: T,
    // value: Option<U>,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
    where T: Fn(U) -> U, 
          U: Hash + Eq + Copy,
{
    fn new(calculation: T) -> Self {
        Cacher { calculation, value: HashMap::new() }
    }

    fn get_value(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                // 不用括包包起来说明使用了一个实现的方法
                let v= (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.get_value(intensity));
        println!("Next, do {} situps!", expensive_result.get_value(intensity));
        // println!("Today, do {} pushups!", expensive_result.get_value("hello"));
        // println!("Next, do {} situps!", expensive_result.get_value("hello"));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.get_value(intensity)
                // expensive_result.get_value("world")
            );
        }
    }
}

fn closure() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    println!("----start----");
    generate_workout(simulated_user_specified_value, simulated_random_number);
    println!("----end----");
}

fn iterator() {
    let shoe = vec![6, 7, 8];

    let filter: Vec<i32> = shoe.into_iter().filter(|size| size % 2 == 0).collect();

    for ele in filter.iter() {
        println!("{}", ele);
    }
}

struct Counter {
    start: u32,
    end: u32,
}

fn range(start: u32, end: u32) -> Counter {
    Counter { start, end }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.start += 1;
            Some(self.start - 1)
        } else {
            None
        }
    }
}

fn imple_iterator() {
    for ele in range(1, 5) {
        print!("{} ", ele);
    }

    println!();
}

enum List {
    // Cons(i32, List),    // 错误
    Cons(i32, Box<List>),
    Nil,
}

fn box_t() {
    use List::{Cons, Nil};

    let list_node = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}


use std::ops::Deref;
struct MyBox<T>(T); // struct 实现的 tuple, 见: tuple_struct

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn self_smart_pointer() {
    let x = MyBox::new(5);
    let y = 5;

    // *x = *(x.deref())
    assert_eq!(y, *x);

    // 隐式类型转换
    let m = MyBox::new(String::from("Rust"));
    hello(&m);


    // 可以实现 Drop 这个 trait, 但是不可以 .drop() 提前释放, 会导致 double free
    // 如果确需要提前释放, 使用 std::mem::drop 释放
}

enum rcList {
    Cons(i32, Rc<rcList>),
    Nil,
}
use crate::rcList::{Cons, Nil};
use std::rc::{Rc, Weak};
fn refer_count() {  //  编译时确定
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("after create a = {}", Rc::strong_count(&a));  // 1

    let b = Cons(3, Rc::clone(&a)); // 使用 Rc::clone(), a 的引用计数加 1 
    println!("after crate b = {}", Rc::strong_count(&a));   // 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("after create c = {}", Rc::strong_count(&a));  // 3
    }

    println!("after c leaves = {}", Rc::strong_count(&a));  // 4
}


use std::cell::RefCell;

#[derive(Debug)]
enum RefCellRcList {
    Cons(i32, RefCell<Rc<RefCellRcList>>),
    Nil
}

impl RefCellRcList {

    fn next(&self) -> Option<&RefCell<Rc<Self>>> {
        match self {
           RefCellRcList::Cons(_, item) => Some(item),
           RefCellRcList::Nil => None,
        }
    }
}

fn reference_cycle() {
    use RefCellRcList::*;

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a rc count: {}", Rc::strong_count(&a));
    println!("a next item: {:?}", a.next());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("b rc count: {}", Rc::strong_count(&b));
    println!("a rc count: {}", Rc::strong_count(&a));
    println!("b next item: {:?}", b.next());

    if let Some(link) = a.next() {
        *link.borrow_mut() = Rc::clone(&b); // b-->a-->b
    }
    println!("b rc count: {}", Rc::strong_count(&b));
    println!("a rc count: {}", Rc::strong_count(&a));
    // println!("a next item: {:#?}", a.next());    // 不能打出来, 否则循环引用
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn weak_reference() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // let p: Option<Rc<Node>> = leaf.parent.borrow().upgrade();
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
}

struct SelfRef {
    value: String,
    pointer_to_value: NonNull<String>,
    _pin: PhantomPinned,
}

fn selfref() {
    let s = "aaa".to_string();
    let v = SelfRef {
        value: s,
        pointer_to_value: NonNull::dangling(),
        _pin: PhantomPinned,
    };

    let mut boxed = Box::pin(v);
    let slice = NonNull::from(&boxed.value);

    unsafe {
        let mut_ref: Pin<&mut SelfRef> = Pin::as_mut(&mut boxed);
        Pin::get_unchecked_mut(mut_ref).pointer_to_value = slice
    }
}


use std::thread;
use std::sync::mpsc::{self, Sender};
fn use_thread_channel() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("a"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });


    // let mut msg = rx.recv().unwrap();
    // msg.push_str(", world");
    // println!("got: {}", msg);

    for mut msg in rx {
        msg.push_str("😂");
        println!("got: {}", msg);
    }

}


use std::sync::Mutex;
use std::sync::Arc; // arc 原子引用计数, 可以在线程间安全并发
use std::sync::MutexGuard;
fn use_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = counter.clone(); 也可以
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num: MutexGuard<i32> = counter.lock().unwrap();
            *num += 1;  // metux 提供了内部可变性
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}

trait Draw {
    fn draw(&self);
}

// trait objects must include the `dyn` keyword
// 当使用 trait 对象时，Rust 必须使用动态分发
struct Screen(Vec<Box<dyn Draw>>);
impl Screen {
    fn run(&self) {
        for component in self.0.iter() {
            component.draw();
        }
    }
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw a button.");
        println!("width: {}", self.width);
        println!("height: {}", self.height);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw a select box.");
        println!("width: {}", self.width);
        println!("height: {}", self.height);
    }
}

fn oop() {
    let screen = Screen(vec![
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),

        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("No"),
            ]
        })
    ]);

    screen.run();
}

fn oop_simple() {
    let mut post = Post::new();
    post.add_text("hello, world");
    println!("post after add text: {}", post.content());

    post.request_review();
    println!("post after request review: {}", post.content());

    post.approve();
    println!("post after approve: {}", post.content());
}

// 不安全函数体也是有效的 unsafe 块，
// 所以在不安全函数中进行另一个不安全操作时无需新增额外的 unsafe 块。
unsafe fn dangerous() {}

fn raw_pointer() {
    // let addr = 0x012345usize;
    let mut addr = 5;

    // 可以在安全代码中 创建 裸指针
    // 不能在不安全块之外 解引用 裸指针
    let r1: *const i32 = &addr as *const i32;
    let r2: *mut i32 = &mut addr as *mut i32;

    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];
    let (f, b) = split_at_mut(r, 3);
    println!("front: {:?}", f);
    println!("back: {:?}", b);
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr: *mut i32 = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid)
        )
    }
}

static mut COUNTER: u32 = 0;

fn static_variable() {
    for _ in 0..10 {
        unsafe {
            COUNTER += 1;
        }
    }

    // 读取也是要 unsafe 的
    unsafe {
        println!("now COUNTER: {}", COUNTER);
    }
}


use std::ops::Add;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    //<Rhs=Self>  默认类型参数
    // rhs 不指定将使用默认类型
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn oprator() {
    let p1 = Point {x: 1, y: 1};
    let p2 = Point {x: 2, y: 2};

    println!("p1 + p2 = {:?}", p1 + p2);
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {

}

fn super_trait() {
    let p = Point {
        x: 1,
        y: 2,
    };

    p.outline_print();

}

// 函数指针也实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce）
type f_add = fn(i32, i32) -> i32;

fn do_times(f: f_add, args: (i32, i32), times: i32) -> i32 {
// fn do_times(f: fn(i32, i32) -> i32, args: (i32, i32), times: i32) -> i32 {
    let mut ret: i32 = 0;

    for _ in 0..times {
        ret += f(args.0, args.1);
    }

    ret
}

fn func_pointer() {
    let res = do_times(add, (1,2), 3);
    println!("res: {}", res);
}

fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn t() {
    let f = returns_closure();
    let s = f(5);
    println!("{}", s);
}

use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

impl Song {
    async fn new() -> Self {
        Self {
            author: "曲婉婷".to_string(),
            name: "我的歌声里".to_string(),
        }
    }

    async fn sing_song(&self) {
        println!("给大家献上一首{}的{}", self.author, self.name);
    }
}

async fn dance() {
    println!("you can relly dance~");
}

async fn learn_and_sing() {
    let song = Song::new().await;
    song.sing_song().await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn use_async() {
    block_on(async_main());
}

use timer_future;

fn main() {
    // 1. 变量使用
    variable_test();

    // // 2. 数据类型
    // typeofdata();
    // arra_y();

    // 3. 函数
    func();

    // 4. 控制流
    judge(-2);
    cycle();
    // label();
    // use_while();
    for_in();

    // 5. 所有权
    ownership();

    // 6. 引用
    reference();
    slice();

    // 7. 结构体
    structure();
    tuple_struct();
    struct_simple();

    // 8. 方法
    method();

    // 9. 枚举
    enumerate();
    if_let();
    while_let();

    // 10. module
    // module();

    // 11. 常用类型
    vector();
    string();
    哈希();

    // 12. 错误处理
    error_handle();
    // error_handle_with_wrap();

    // 13. 范型
    generic_type();
    // 14. trait
    impl_trait();
    // 15. 生命周期 
    lifetime();
    // 16. 例子
    simple();

    // 17. test

    // 18. 闭包
    // closure();

    // 19. 迭代器
    iterator();
    imple_iterator();

    // 20. 文档注释
    fust_tutorials::add_two(5);

    // 21. Box<T>
    box_t();

    // 22. 自己实现的智能指针
    // 在于实现了 Deref 这个 trait
    self_smart_pointer();

    // 23. 引用计数
    refer_count();

    // 24. 循环引用
    reference_cycle();
    // 25. 弱引用
    weak_reference();
    // 自引用
    selfref();

    // 26. channel
    use_thread_channel();
    // 27. 互斥锁
    use_mutex();

    // 28. 面向对象
    oop();
    oop_simple();

    // 29. 裸指针
    raw_pointer();
    // 30. 定义和使用全局静态变量
    static_variable();
    
    // 31. 重载运算符
    oprator();
    // 32.
    // 33. 父 trait
    super_trait();

    // 33. 函数指针
    func_pointer();

    t();

    // 34. async/await
    use_async();
}
