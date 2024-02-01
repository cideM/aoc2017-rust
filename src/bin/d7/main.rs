use std::collections::HashMap;
use std::io;

fn main() {
    // let mut tree = HashMap::new();
    for line in io::stdin().lines() {
        let s = line.unwrap();
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        match parts.as_slice() {
            [name, weight, _, children @ ..] => {
                let without_comma = children
                    .iter()
                    .map(|s| s.replace(",", ""))
                    .collect::<Vec<_>>();
                println!("{:?}", without_comma);
            }
            [name, weight] => {
                println!("{} {}", name, weight);
            }
            x => {
                println!("couldn't parse: {:?}", x);
                std::process::exit(1);
            }
        }
    }
}
