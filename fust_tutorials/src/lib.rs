// mod front_of_house 后使用分号，而不是代码块
// 这将告诉 Rust 在另一个与模块同名的文件中加载模块的内容
mod front_of_house;

pub use front_of_house::hosting;


pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


// 定义一组行为
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 默认行为, 可以重写
    fn summarize(&self) -> String {
        // 因为不知道谁实现了 trait, 所以不能用 self 的成员
        format!("Read more from {}...", self.summarize_author())
    }
}

struct News {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub author: String,
    pub content: String,
    pub reply_number: usize,
    pub can_retweet: bool,
}


impl Summary for News {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }

    // 重写默认实现
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32
}

impl Rectangle {
    // 可简写为 &self
    pub fn area(self: &Self) -> u32 {
        self.height * self.height
    }

    pub fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

mod guess;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5, 
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    fn greeting(name: &str) -> String {
        format!("heelo,{}", name)
    }

    #[test]
    fn contain_name() {
        let result = greeting("dawson");

        assert!(
            result.contains("dawson"),
            "not contain nmae, value was {}",
            result
        );
    }

    use guess::Guess;

    #[test]
    // #[should_panic]    // 都可以
    #[should_panic(expected = "Guess value must be between 1 - 100")]   // 信息是 panic 的字串
    fn greter_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}

/// Add two to the number given.
/// 
/// # Examples
/// ```
/// let arg = 5;
/// let answer = fust_tutorials::add_two(arg);
/// 
/// assert_eq!(7, answer);
/// 
pub fn add_two(num: i32) -> i32 {
    num + 2
}



pub trait Messager {
    fn send(&self, msg: &str);
}



pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where 
    T: Messager,
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker { messager , value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;

        if percentage >= 1.0 {
            self.messager.send("Error: You are ouver your quota!");
        } else if percentage >= 0.75 {
            self.messager.send("Warning: you've used up over 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod test_messager {
    use super::*;
    use std::cell::RefCell;

    struct MockMessager {
        // 内部不可变
        // sent_messages: Vec<String>

        // 内部可变
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessager {
        fn new() -> Self {
            // MockMessager { sent_messages: vec![] }

            MockMessager { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {
            // 这里因为 self 是一个不可变引用, 所以无法 push
            // self.sent_messages.push(String::from(msg));

            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn tt() {
        let mock_msg = MockMessager::new();
        let mut limit = LimitTracker::new(&mock_msg, 100);

        limit.set_value(80);

        assert_eq!(mock_msg.sent_messages.borrow().len(), 1);
    }
}



pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self { state: Some(Box::new(Draft {})), content: String::new() }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        // 调用 take 方法将 state 字段中的 Some 值取出并留下一个 None
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}


struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}