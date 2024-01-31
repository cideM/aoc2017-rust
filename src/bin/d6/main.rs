use std::collections::HashSet;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut banks = input
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();

    // store seen state in hash set
    // find index of max value; prefer smaller index in case of tie
    // store value in variable; empty that index in vec
    // while value > 0, move to the next position and inc by 1, dec value by 1
    // if current state is seen, done
    // if not, add to state, done

    // TODO: why can't we from::(seen)
    let mut seen = HashSet::new();

    seen.insert(banks.clone()); // borrow checker moves this because https://docs.rs/hashbrown/latest/src/hashbrown/map.rs.html#1751-1766


    let mut max_value = usize::MIN;
    let mut position = 0;
    for (i, n) in banks.iter().enumerate() {
        if *n > max_value {
            max_value = *n;
            position = i;
        }
    }

    println!("{:?}", banks);
    banks[position] = 0;

    for i in (0..banks.len()).cycle().skip(position + 1) {
        if max_value == 0 {
            break;
        }
        max_value -= 1;
        banks[i] += 1;
    }

}
