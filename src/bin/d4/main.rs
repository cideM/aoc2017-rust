use std::collections::HashSet;
use std::io;
use std::ops::ControlFlow;

fn line_has_duplicates(line: &str) -> bool {
    line.split_whitespace()
        .try_fold(HashSet::new(), |mut acc, x| {
            if acc.contains(x) {
                ControlFlow::Break(acc)
            } else {
                acc.insert(x);
                ControlFlow::Continue(acc)
            }
        })
        .is_continue()
}

fn main() {
    let p1 = io::stdin()
        .lines()
        .map(|result| result.unwrap())
        .filter(|s| line_has_duplicates(s)) // TODO: why not filter(line_has_duplicates)
        .collect::<Vec<_>>()
        .len();
    println!("{:?}", p1);

    let p2 = io::stdin()
        .lines()
        .map(|result| {
            let s = result.unwrap();
			println!("{}", s);
            let mut sorted: Vec<String> = Vec::new();
            for word in s.split_whitespace() {
                let mut chars: Vec<char> = word.chars().collect();
                chars.sort();
                sorted.push(chars.into_iter().collect());
            }
			println!("{}", sorted.join(" "));
            sorted.join(" ")
        })
        .filter(|s| line_has_duplicates(s)) // TODO: why not filter(line_has_duplicates)
        .collect::<Vec<_>>()
        .len();
    println!("{:?}", p2);
}
