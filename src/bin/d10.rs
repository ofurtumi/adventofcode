use crate::standard::read_lines;
pub mod standard;

fn main() {
    let mut cycle = 0;
    let mut reg_x = 1;
    let mut syg_strength = 0;
    let lines = read_lines();
    for line in lines {
        let command: Vec<&str> = line.split(" ").collect();
        match command[0] {
            "noop" => cycle += 1,
            "addx" => {
                cycle += 1;
                if (cycle % 40) - 20 == 0 {
                    let c_syg_strength = cycle * reg_x;
                    syg_strength += c_syg_strength;
                    println!(
                        "cycle: {:4}, value a: {:4}, total: {:6}",
                        cycle, reg_x, c_syg_strength
                    )
                }
                cycle += 1;
                reg_x += command[1].parse::<i32>().unwrap();
            }
            _ => (),
        }
        if (cycle % 40) - 20 == 0 {
            let c_syg_strength = cycle * reg_x;
            syg_strength += c_syg_strength;

            println!(
                "cycle: {:4}, value b: {:4}, total: {:6}",
                cycle, reg_x, c_syg_strength
            )
        }
    }
    print!("{syg_strength}")
}
