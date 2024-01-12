use std::io;
use std::iter;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let nums: Vec<u32> = buffer.chars().map(|x| x.to_digit(10).unwrap()).collect();

    let iter1 = nums.iter();
    let iter2 = iter1.clone().cycle().skip(1);
    let p1: u32 = iter::zip(iter2, iter1)
        .map(|(x, y)| if x == y { *x } else { 0 })
        .sum();
    println!("{}", p1);

    let mut nums2 = nums.clone();
    nums2.rotate_right(nums.len() / 2);
    let iter4 = nums2.iter().cycle();
    let p2: u32 = iter::zip(iter4, nums.iter())
        .map(|(x, y)| if x == y { *x } else { 0 })
        .sum();
    println!("{}", p2);
    Ok(())
}
