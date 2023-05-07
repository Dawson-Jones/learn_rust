#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}


impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}


fn main() {
    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "hello", y: true};

    let p3 = p1.mixup(p2);
    println!("{:#?}", p3);

    let p = Point {x: 3.0_f32, y: 4.0_f32};
    println!("distance_from_origin: {}", p.distance_from_origin());

    // const 泛型, 针对值的泛型
    let arr:[i32; 3] = [1,2,3];
    display_array(arr);
    let arr:[i32; 2] = [1,2];
    display_array(arr);
}