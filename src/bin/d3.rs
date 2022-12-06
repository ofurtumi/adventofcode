use crate::standard::read_lines;

pub mod standard;

fn main() {
    static ASCII: [char; 53] = [
        ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
        'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let lines = read_lines();

    let mut out = 0;

    for line in lines {
        let length = line.len() / 2;
        let (front, end) = line.split_at(length);
        let chars: Vec<char> = end.chars().collect();

        // println!("{:?}", chars);
        // println!("{:?}", front.contains(chars[3]));

        for character in chars {
            let found = front.contains(character);
            if found {
                let point = ASCII.iter().position(|&n| n == character);
                println!("{character} : {:?}", point);
                out += point.unwrap();
                break;
            };
        }
    }

    print!("{out}");
}
