use crate::standard::read_lines;
pub mod standard;

use take_until::TakeUntilExt;

fn main() {
    let lines: Vec<Vec<u32>> = read_lines()
        .iter()
        .map(|w| w.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let size = lines.len();
    let line_size = lines[0].len();
    let mut tree_hits = vec![vec![0; lines.len()]; lines.len()];

    for line in 0..size {
        tree_hits[0][line] = 1;
        tree_hits[size - 1][line] = 1;
        tree_hits[line][0] = 1;
        tree_hits[line][size - 1] = 1;

        let mut _largest = lines[line][0];

        for tree in 1..line_size {
            if lines[line][tree] > _largest {
                _largest = lines[line][tree];
                tree_hits[line][tree] = 1;
            }
        }

        _largest = lines[line][line_size - 1];

        for tree in (0..line_size - 1).rev() {
            if lines[line][tree] > _largest {
                _largest = lines[line][tree];
                tree_hits[line][tree] = 1;
            }
        }

        _largest = lines[0][line];

        for tree in 0..line_size - 1 {
            if lines[tree][line] > _largest {
                _largest = lines[tree][line];
                tree_hits[tree][line] = 1;
            }
        }
        _largest = lines[line_size - 1][line];

        for tree in (1..line_size).rev() {
            if lines[tree][line] > _largest {
                _largest = lines[tree][line];
                tree_hits[tree][line] = 1;
            }
        }
    }
    // println!("{}", vis.len());
    // for i in &tree_hits {
    //     println!("{i:?}");
    // }
    // println!();
    // for i in &lines {
    //     println!("{i:?}");
    // }

    let mut top_view: (usize, usize, usize, usize, usize, usize) = (0, 0, 0, 0, 0, 0);

    for i in 0..size {
        for j in 0..size {
            let n = lines[..i]
                .iter()
                .map(|v| v[j])
                .take_until(|n| *n >= lines[i][j])
                .count();
            
            let w = lines[i][..j]
                .iter()
                .copied()
                .rev()
                .take_until(|n| *n >= lines[i][j])
                .count();

            let s = lines[i + 1..]
                .iter()
                .map(|v| v[j])
                .rev()
                .take_until(|n| *n >= lines[i][j])
                .count();

            let e = lines[i][j + 1..size]
                .iter()
                .copied()
                .take_until(|n| *n >= lines[i][j])
                .count();

            let temp_greatest = n * e * s * w;
            if temp_greatest > (top_view.0 * top_view.1 * top_view.2 * top_view.3) {
                top_view = (n, w, s, e, j, i)
            }
        }
    }

    let mut total_trees = 0;
    for i in &tree_hits {
        total_trees += i.iter().filter(|&n| *n == 1).count();
    }

    println!("visible trees: {total_trees}");
    println!("top location data:  {top_view:?}");
    println!(
        "top location calc:  {}",
        (top_view.0 * top_view.1 * top_view.2 * top_view.3)
    );
}
