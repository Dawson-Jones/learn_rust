macro_rules! make_local {
    () => {
        local = 42;
    };
}

fn main() {
    let mut local = 0;
    make_local!();
    assert_eq!(local, 0);
}