use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::ops::ControlFlow;

// Decent explanation of into_iter vs iter check accepted answer:
// https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter

fn find_duplicate_index<T: Eq + Hash>(v: impl IntoIterator<Item = T>) -> Option<usize> {
    match v
        .into_iter()
        .enumerate()
        .try_fold(HashSet::new(), |mut acc, (i, x)| {
            if acc.contains(&x) {
                ControlFlow::Break(i)
            } else {
                acc.insert(x);
                ControlFlow::Continue(acc)
            }
        }) {
        ControlFlow::Continue(_) => None,
        ControlFlow::Break(i) => Some(i),
    }
}

fn sort_chars(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.iter().collect()
}

fn main() {
    let mut p1 = 0;
    let mut p2 = 0;
    for result in io::stdin().lines() {
        match result {
            Ok(line) => {
                let words = line.split_whitespace();
                if let None = find_duplicate_index(words.clone()) {
                    p1 += 1;
                }

                if let None = find_duplicate_index(words.map(sort_chars)) {
                    p2 += 1;
                }
            }
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        }
    }

    println!("{} {}", p1, p2);
}
