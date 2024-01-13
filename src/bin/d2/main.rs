use std::cmp;
use std::io;

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in io::stdin().lines() {
        let nums = line
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut max = usize::MIN;
        let mut min = usize::MAX;
        for (i, num) in nums.iter().enumerate() {
            max = cmp::max(max, *num);
            min = cmp::min(min, *num);
            for n2 in &nums[i + 1..] {
                let high = cmp::max(num, n2);
                let low = cmp::min(num, n2);
                if high % low == 0 {
                    p2 += high / low;
                }
            }
        }
        p1 += max - min;
    }
    println!("{} {}", p1, p2);
}
