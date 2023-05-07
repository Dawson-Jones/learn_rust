use std::{
    env,
    fs::File,
    io::{self, BufRead, Read},
    path::Path, num,
};

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let re = regex::Regex::new(r"([0-9]+\.[0-9]+)|(\d+)").unwrap();
    let mut total = 0_f64;
    match read_lines("./text.txt") {
        Ok(lines) => {
            for line in lines {
                let s = line.unwrap();
                let s = s.trim();
                println!("s: {}", s);
                for capture in re.captures_iter(s) {
                    match capture.get(0) {
                        Some(capture) => {
                            let number = capture.as_str().parse::<f64>().unwrap();
                            println!("The number is {}", number);
                            total += number;
                        },
                        None => println!("nothing matched"),
                    }
                }
            }
        },
        Err(e) => println!("read_lines: {}", e.to_string()),
    }


    println!("total: {}", total)
}
