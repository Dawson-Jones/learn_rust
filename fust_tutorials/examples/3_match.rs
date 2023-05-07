enum Message {
    Hello { id: i32 },
} 

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point {x: a, y: b} = p;
    // let Point {x, y} = p;
    assert!(a == 0);
    assert!(b == 7);

    // 用 @ 绑定
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id@ 3..=7 } => {
            println!("Found an id in range: {}", id)
        },
        Message::Hello { id: id @ (10|11|12) } => {
            println!("Found an id in another range: {}", id)
        },
        Message::Hello { id: (13..=16) } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
