use std::collections::HashSet;
use std::io;

fn steps_until_repeat(init: Vec<usize>) -> (Vec<usize>, usize) {
    let mut banks = init.clone();

    // TODO: why can't we from::(seen)
    let mut seen = HashSet::new();
    seen.insert(banks.clone()); // borrow checker moves this because https://docs.rs/hashbrown/latest/src/hashbrown/map.rs.html#1751-1766

    loop {
        let mut max_value = usize::MIN;
        let mut position = 0;
        for (i, n) in banks.iter().enumerate() {
            if *n > max_value {
                max_value = *n;
                position = i;
            }
        }

        banks[position] = 0;

        for i in (0..banks.len()).cycle().skip(position + 1) {
            if max_value == 0 {
                break;
            }
            max_value -= 1;
            banks[i] += 1;
        }
        if seen.contains(&banks) {
            return (banks.clone(), seen.len());
        }
        seen.insert(banks.clone());
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let banks = input
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();
    let (repeated_state, steps) = steps_until_repeat(banks);
    println!("{}", steps);
    let (_, steps_p2) = steps_until_repeat(repeated_state);
    println!("{}", steps_p2);
}
