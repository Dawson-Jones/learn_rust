use std::slice;
use std::{str::from_utf8_unchecked, slice::from_raw_parts};

/// unsafe 的能力
/// - 解引用裸指针
/// - 调用 unsafe 或外部的函数
/// - 访问或修改一个可变的静态变量
/// - 实现一个 unsafe 特征
/// - 访问 union 中的字段

fn get_memory_location() -> (usize, usize) {
    let s = "hello, world";
    let p = s.as_ptr() as usize;
    let l = s.len();

    (p, l)
}

fn get_str_at_location(p: usize, l: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked(from_raw_parts(p as *const u8, l))
    }
}

fn main() {
    let nums = [5, 4, 3];
    let p = &nums as *const i32;
    unsafe {
        let p2 = p.add(2);
        // 解引用裸指针
        println!("{}", (*p2));
    }

    let (p, l) = get_memory_location();
    let msg = get_str_at_location(p, l);
    println!(
        "The {} bytes at 0x{:X} stored {}",
        l, p, msg
    );

    // 调用 unsafe 函数
    unsafe { dangerous() }
    // 获得两个可变引用
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    
    // FFI: 
    // - rs 调用 c: 见 RScallC
    // - c 调用 rs: 见 CcallRS

    // 访问 union 中的字段
}

unsafe fn dangerous() {}

fn split_at_mut(arr: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = arr.len();
    assert!(mid < len);

    // let p1 = &mut slice[0] as *mut i32;
    // let p2 = &mut slice[mid] as *mut i32;

    unsafe {
        let p1 = arr.as_mut_ptr();
        let p2 = p1.add(mid);   // p1 + mid * sizeof(i32), 不知道 sizeof 方法

        (
            slice::from_raw_parts_mut(p1, mid),
            slice::from_raw_parts_mut(p2, len - mid),
        )
    }
}