use std::{
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut top_3:Vec<i32> = Vec::new();
    let mut out = 0;

    while let Some(line) = lines.next() {
        let this_line = line.unwrap();

        if this_line.trim().is_empty() {
            top_3.push(out);
            out = 0;
        } else {
            out += this_line.trim().parse::<i32>().unwrap();
        }
    }

    top_3.sort_by(|a,b| b.cmp(a));

    out = 0;
    for i in 0..3 {
        out += top_3[i];
    }
    println!("{out}");
    Ok(())
}
