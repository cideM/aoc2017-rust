use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

fn compute_weights(
    parent_child: HashMap<String, Vec<String>>,
    weights: HashMap<String, usize>,
) -> Option<usize> {
	Some(1)
}

fn main() {
    let mut weights: HashMap<String, usize> = HashMap::new();
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    for line in io::stdin().lines() {
        let mut s = line.unwrap();
        s.retain(|c| ![',', ')', '('].contains(&c));

        let parts = s.split_whitespace().collect::<Vec<&str>>();

        match parts.as_slice() {
            // TODO: Deal with duplication
            [name, weight, _, children @ ..] => {
                let owned_children: Vec<String> =
                    children.iter().map(|s| (*s).to_owned()).collect();

                let name_owned = (*name).to_owned();

                let weight_parsed: usize = weight.parse().unwrap();
                weights.insert(name_owned.clone(), weight_parsed);

                if let Some(_) = tree.insert(name_owned, owned_children) {
                    println!("key {} was already in map", name);
                    std::process::exit(1);
                }
            }
            [name, weight] => {
                let name_owned = (*name).to_owned();

                if let Some(_) = tree.insert(name_owned.clone(), vec![]) {
                    println!("key {} was already in map", name);
                    std::process::exit(1);
                }

                let weight_parsed: usize = weight.parse().unwrap();
                weights.insert(name_owned.clone(), weight_parsed);
            }
            x => {
                println!("couldn't parse: {:?}", x);
                std::process::exit(1);
            }
        }
    }

    let all_keys = tree.keys().collect::<HashSet<&String>>();
    let all_children = tree.values().flatten().collect::<HashSet<&String>>();
    println!("{:?}", all_keys.difference(&all_children));

    // TODO: recurse
}
