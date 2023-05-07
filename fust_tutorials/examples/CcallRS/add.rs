// 将该代码编译成共享库, 链接到 C 语言中
#[no_mangle]    // 不要混乱函数的名称
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}