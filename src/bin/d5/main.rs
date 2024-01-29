use std::io;

fn main() {
	// TODO: Remove some of the duplication

    let mut nums = io::stdin()
        .lines()
        .map(|line| match line {
            Ok(s) => match s.parse::<isize>() {
                Ok(num) => num,
                Err(e) => {
                    println!("failed to parse: {}; error: {}", s, e);
                    std::process::exit(1);
                }
            },
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        })
        .collect::<Vec<isize>>();

	let mut nums_p1 = nums.clone();
    let mut position = 0;
    let mut steps = 0;
    let length = nums_p1.len();
    while position < length {
        let cur = nums_p1[position];
        match position.checked_add_signed(cur) {
            None => {
                // TODO: document
                break;
            }
            Some(next_pos) => {
                nums_p1[position] += 1;
                position = next_pos;
                steps += 1;
            }
        }
    }
    println!("{}", steps);

    let mut position = 0;
    let mut steps = 0;
    while position < length {
        let cur = nums[position];
        match position.checked_add_signed(cur) {
            None => {
                // TODO: document
                break;
            }
            Some(next_pos) => {
                if cur >= 3 {
                    nums[position] -= 1;
                } else {
                    nums[position] += 1;
                }
                position = next_pos;
                steps += 1;
            }
        }
    }
    println!("{}", steps);
}
