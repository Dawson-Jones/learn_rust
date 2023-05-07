use chrono::Utc;
use once_cell::sync::Lazy;

static GLOBAL_DATA: Lazy<String> = Lazy::new(||Utc::now().to_string());

fn main() {
    print!("{}", *GLOBAL_DATA);
}