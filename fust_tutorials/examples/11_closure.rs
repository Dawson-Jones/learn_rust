use std::vec;

fn fn_once<F>(func: F) 
    where F: FnOnce(usize)->bool + Copy
{
    // FnOnce 只能被使用一次, 所有权被转移
    // 所以要求 FnOnce 实现了 Copy, 这样就可以在使用的时候进行 Copy 了
    println!("{}", func(3));
    println!("{}", func(4));
}


fn main() {
    let x = vec![1,2,3];
    // 这里的 x.len 使用了 x 的引用, 如果要获取 x 的所有权, 要使用 move
    let fn_one = /* move */ |z: usize| {z == x.len()};
    fn_once(fn_one);


    // FnMut: 以可变借用的方式捕获环境中的值
    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{}", s)
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello");
}