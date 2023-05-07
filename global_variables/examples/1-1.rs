// use chrono::Utc;


// static START_TIME: String = Utc::now().to_string();

// fn main() {
//     let thread_1 = std::thread::spawn(|| {
//         println!("startd {}, called thread 1 {}", START_TIME.as_ref().unrap(), Utc::now());
//     });

//     let thread_2 = std::thread::spawn(|| {
//         println!("startd {}, called thread 2 {}", START_TIME.as_ref().unrap(), Utc::now());
//     });

//     thread_1.join().unwrap();
//     thread_2.join().unwrap();
// }