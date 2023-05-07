use std::{fmt::Display, io, fs::{File, read_to_string}};



// #[derive(thiserror::Error)]
#[derive(Debug)]
struct MyError {
    code: usize,
    message: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, error: {}", self.code, self.message)
    }
}

// 为 io::Error 实现向 MyError 的转换
impl From<io::Error> for MyError {
    fn from(value: io::Error) -> Self {
        Self { code: 1, message:  value.to_string() }
    }
}

fn produce_error() -> Result<(), MyError> {
    Err(MyError {
        code: 404,
        message: String::from("Page not found")
    })
}

fn main() -> Result<(), MyError> {
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error()); // Err({ file: src/main.rs, line: 17 })

    //  ? 可以将错误进行隐式的强制转换
    let _f = File::open("not_exist_file.txt")?;

    Ok(())

    // 归一化错误处理
    // 1. 使用特征对象 Box<dyn std::error::Error>, 需要实现Debug + Display
    // 2. 自定义错误类型, 啰嗦
    // 3. 使用 thiserror
}


// fn render() -> Result<String, MyError> {
//     let file = std::env::var("MARKDOWN")?;
//     let source = read_to_string(file)?;
//     Ok(source)
// }