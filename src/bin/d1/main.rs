use std::io;
use std::iter;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let nums: Vec<u32> = buffer.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let p1: u32 = iter::zip(nums.iter().cycle().skip(1), nums.iter())
        .map(|(x, y)| if x == y { *x } else { 0 })
        .sum();

    let mut five_ahead = nums.clone();
    five_ahead.rotate_right(nums.len() / 2);
    let p2: u32 = iter::zip(five_ahead.iter().cycle(), nums.iter())
        .map(|(x, y)| if x == y { *x } else { 0 })
        .sum();
    println!("{} {}", p1, p2);
}
