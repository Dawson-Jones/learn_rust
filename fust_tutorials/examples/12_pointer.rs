fn main() {
    // raw pointer
    let mut values = [1,2];
    let p1 = values.as_mut_ptr();
    let p1_addr = p1 as usize;
    let p2_addr = p1_addr + 4;
    let p2 = p2_addr as *mut i32;

    unsafe {
        (*p2) += 1;
    }

    assert_eq!(values[1], 3);

    // try_into
    let b: i32 = 1500;
    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => { // u8 无法承载 1500
            eprintln!("{:?}", e.to_string());
            0
        }
    };
}