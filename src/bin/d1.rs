use crate::standard::read_lines;
pub mod standard;

fn main() {
    let lines = read_lines();
    let mut top_3: Vec<i32> = Vec::new();
    let mut out = 0;

    for line in lines {
        if line.trim().is_empty() {
            top_3.push(out);
            out = 0;
        } else {
            out += line.trim().parse::<i32>().unwrap();
        }
    }

    top_3.sort_by(|a, b| b.cmp(a));
    println!("{:?}", top_3[0])

    
    // ? fyrir seinni hlutann
    // out = 0;
    // for i in 0..3 {
    //     out += top_3[i];
    // }
    // println!("{out}");
}
