use crate::standard::read_lines;

pub mod standard;

fn main() {
    let mut arr1 = vec!['J', 'H', 'G', 'M', 'Z', 'N', 'T', 'F'];
    let mut arr2 = vec!['V', 'W', 'J'];
    let mut arr3 = vec!['G', 'V', 'L', 'J', 'B', 'T', 'H'];
    let mut arr4 = vec!['B', 'P', 'J', 'N', 'C', 'D', 'V', 'L'];
    let mut arr5 = vec!['F', 'W', 'S', 'M', 'P', 'R', 'G'];
    let mut arr6 = vec!['G', 'H', 'C', 'F', 'B', 'N', 'V', 'M'];
    let mut arr7 = vec!['D', 'H', 'G', 'M', 'R'];
    let mut arr8 = vec!['H', 'N', 'M', 'V', 'Z', 'D'];
    let mut arr9 = vec!['G', 'N', 'F', 'H'];

    let lines = read_lines();

    for line in lines {
        let ctrl: Vec<u32> = line
            .replace("move ", "")
            .replace("from ", "")
            .replace("to ", "")
            .split(" ")
            .map(|c| c.parse::<u32>().unwrap())
            .collect();

        let from = match ctrl[1] {
            1 => &mut arr1,
            2 => &mut arr2,
            3 => &mut arr3,
            4 => &mut arr4,
            5 => &mut arr5,
            6 => &mut arr6,
            7 => &mut arr7,
            8 => &mut arr8,
            9 => &mut arr9,
            n => panic!("ekki til, {}", n),
        };

        let mut station: Vec<char> = vec![];
        for _i in 0..ctrl[0] {
            station.push(from.pop().unwrap())
        }
        station.reverse();

        let to = match ctrl[2] {
            1 => &mut arr1,
            2 => &mut arr2,
            3 => &mut arr3,
            4 => &mut arr4,
            5 => &mut arr5,
            6 => &mut arr6,
            7 => &mut arr7,
            8 => &mut arr8,
            9 => &mut arr9,
            _ => panic!("ekki til"),
        };

        for _x in 0..ctrl[0] {
            to.push(station.pop().unwrap());
        }
    }

    println!("{arr1:?}");
    println!("{arr2:?}");
    println!("{arr3:?}");
    println!("{arr4:?}");
    println!("{arr5:?}");
    println!("{arr6:?}");
    println!("{arr7:?}");
    println!("{arr8:?}");
    println!("{arr9:?}");

}
