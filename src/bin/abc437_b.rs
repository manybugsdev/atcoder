use std::collections::*;

fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut nums = stdin
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let h = nums.remove(0);
    let w = nums.remove(0);
    let n = nums.remove(0);
    let mut grid = Vec::new();
    for _ in 0..h {
        let mut row = HashSet::new();
        for _ in 0..w {
            row.insert(nums.remove(0));
        }
        grid.push(row);
    }
    let mut calls = HashSet::new();
    for _ in 0..n {
        calls.insert(nums.remove(0));
    }
    let mut max = 0;
    for i in 0..h {
        let c = grid[i].intersection(&calls).count();
        if c > max {
            max = c;
        }
    }
    println!("{}", max);
}
