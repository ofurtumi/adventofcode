use std::{fmt::Debug, io::Read, };

pub fn read_lines() -> Vec<String> {
    let mut buffer: Vec<u8> = vec![];
    match std::io::stdin().read_to_end(&mut buffer) {
        Err(e) => panic!("error reading stdin, {}", e),
        _ => (),
    };

    let out:Vec<String> = match std::str::from_utf8(&mut buffer) {
        Ok(w) => w.trim().split("\n").map(|s| s.to_string()).collect(),
        Err(e) => panic!("error splitting stdin, {}", e),
    };
    out
}

pub fn read_lines_as<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug
{
    let mut buffer: Vec<u8> = vec![];
    match std::io::stdin().read_to_end(&mut buffer) {
        Err(e) => panic!("error reading stdin, {}", e),
        _ => (),
    }
    let out = match std::str::from_utf8(&mut buffer) {
        Ok(w) => w
            .trim()
            .split("\n")
            .map(|line| line.parse::<T>().unwrap())
            .collect::<Vec<T>>(),
        Err(e) => {
            panic!("error parsing stdin, {}", e)
        }
    };
    out
}