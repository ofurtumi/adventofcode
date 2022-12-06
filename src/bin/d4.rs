use crate::standard::read_lines; 

pub mod standard;

fn main() {
    let lines = read_lines();

    let mut count = 0;

    for line in lines {
        // ? hugsanlegur oneliner fyrir inputið
        let pairs: Vec<Vec<u32>> = line
            .split(",")
            .map(|w| {
                w.split("-")
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        if pairs[0][0] <= pairs[1][0] && pairs[1][0] <= pairs[0][1] {
            count += 1
        } else if pairs[0][1] >= pairs[1][1] && pairs[1][1] >= pairs[0][0] {
            count += 1
        } else if pairs[1][0] <= pairs[0][0] && pairs[0][0] <= pairs[1][1] {
            count += 1
        } else if pairs[1][1] >= pairs[0][1] && pairs[0][1] >= pairs[1][0] {
            count += 1
        }
    }

    print!("{count}");
}
