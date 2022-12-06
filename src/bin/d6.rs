use std::{
    collections::HashSet,
};
use crate::standard::read_lines;

pub mod standard;

fn main(){
    const SEARCH_SIZE: usize = 14;

    let lines = read_lines();

    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();

        let result = chars.windows(SEARCH_SIZE).collect::<Vec<_>>();

        let mut cnt = 0;
        for i in result {
            let uniqued_vector = i
                .into_iter()
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();

            if uniqued_vector.len() == SEARCH_SIZE {
                break;
            } else {
                cnt += 1;
            }
        }
        print!("{}", cnt + SEARCH_SIZE);
    }
}
