use crate::standard::read_lines;
pub mod standard;

fn main() {
    let lines = read_lines();
    
    let mut out:i32 = 0;
    for line in lines {
        let mut temp = 0;
        match line.trim() {
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