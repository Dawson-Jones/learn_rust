
fn change(s: &mut String) {
    s.push_str(", World");
}


// 引用必须总是有效的
    // 原值的生命周期要比引用更长
// 可变引用只能存在一个: 避免数据竞争
// 可变引用不能和不可变引用同时存在: 避免不可变引用引用的东西发生变化
fn mutable_reference() {
    let mut s = String::from("Hello");
    change(&mut s);

    println!("s: {s}");
}

fn main() {
    mutable_reference();
}