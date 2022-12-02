use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut out:i32 = 0;
    while let Some(line) = lines.next() {
        let mut temp = 0;
        match line.unwrap().trim() {
            "A X" => temp = 3,
            "A Y" => temp = 4,
            "A Z" => temp = 8,
            "B X" => temp = 1,
            "B Y" => temp = 5,
            "B Z" => temp = 9,
            "C X" => temp = 2,
            "C Y" => temp = 6,
            "C Z" => temp = 7,
            _ => print!("klikkar")
        }

        println!("{temp} + {out} =  {}", temp+out);
        out += temp;
    }
    println!("{}", out)
}
