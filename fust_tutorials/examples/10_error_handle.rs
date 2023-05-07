// 只有当你不知道该如何处理时，再去调用 panic!.

use std::{error::Error, fs::File};

use futures::future::ok;


// 使用 ? 对类型进行隐式转换
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    // io::Result 实现了 std::error::Error 这个 trait 所以进行了自动转换
    let f = File::open("hello.txt")?;   
    Ok(f)
}


fn main() {
    let f = open_file();
    match f {
        Ok(f) => todo!(),
        Err(e) => todo!(),
    }
}
