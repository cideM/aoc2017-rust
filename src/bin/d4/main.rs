use std::collections::HashSet;
use std::io;
use std::ops::ControlFlow;

fn main() {
    let p1 = io::stdin()
        .lines()
        .map(|result| result.unwrap())
        .filter(|s| {
            s.split_whitespace()
                .try_fold(HashSet::new(), |mut acc, x| {
                    if acc.contains(x) {
                        ControlFlow::Break(acc)
                    } else {
                        acc.insert(x);
                        ControlFlow::Continue(acc)
                    }
                })
                .is_continue()
        })
        .collect::<Vec<_>>()
        .len();
    println!("{:?}", p1);
}
