pub struct Guess {
    value: i32
}


impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 - 100, got {}", value)
        }

        Guess {value}
    }
}


