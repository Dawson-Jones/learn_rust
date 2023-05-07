/// 全局变量的生命周期肯定是'static
/// 静态常量, const MAX_ID: usize =  usize::MAX / 2; 就是宏定义, 编译时内联到具体代码
/// 静态变量, static mut REQUEST_RECV: usize = 0;
///     - 必须使用 unsafe 语句块才能**访问**和**修改** static 变量
///     - 定义静态变量的时候必须赋值为在编译期就可以计算出的值(常量表达式/数学表达式)，不能是运行时才能计算出的值(如函数), 不过可以通过 lazy 的方式避免
/// 运行期初始化: lazy_static


use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    // lazy_static宏，匹配的是static ref，所以定义的静态变量都是不可变引用
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Hello, Dawson"));
}

#[derive(Debug)]
struct Config {
    a: String,
    b: String
}
static mut CONFIG: Option<&mut Config> = None;

fn main() {
    let mut v = NAMES.lock().unwrap();
    v.push_str(" Jones");
    println!("{}", v);

    // 主动内存泄漏
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    unsafe {
        // 主动内存泄漏使其变成 ‘static 的生命周期
        CONFIG = Some(Box::leak(c));
    }

    unsafe {
        println!("{:?}", CONFIG);
    }
}