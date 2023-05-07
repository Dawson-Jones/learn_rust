// 1. Rust 每个值只能被一个变量所拥有, 该变量被称为该值的所有者
// 2. 所有者变量离开作用域, 该值被 drop


fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;    // s1 不再有效, 这个操作被称为 move

    // println!("{}, world!", s1);
    //                        ^^ value used here after move

}   // drop s2


// Copy 特征
// 如果一个类型拥有 Copy 特征, 那么旧的变量在被赋值给其他变量后, 仍然可用
// 任何 基本类型 的组合可以 Copy

// 函数的传值和返回会发生 移动 or 复制