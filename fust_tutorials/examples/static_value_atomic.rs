use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use arc_swap::ArcSwap;

static MAP: Lazy<ArcSwap<HashMap<String, String>>> = Lazy::new(|| {
    ArcSwap::from_pointee(HashMap::new())
});



fn main() {
    let _t1 = thread::spawn(|| {
        let mut new_map = HashMap::new();
        new_map.insert("name".to_string(), "dawson".to_string());
        new_map.insert("gender".to_string(), "male".to_string());

        let new_map = Arc::new(new_map);
        MAP.store(new_map);
    });



    loop {
        let m = MAP.load();

        if m.is_empty() {
            thread::sleep(Duration::from_secs(1));
        } else {
            break;
        }
    }

    let m = MAP.load();
    if !m.is_empty() {
        println!("name: {}", m.get("name").unwrap())
    }
}