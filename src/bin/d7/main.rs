use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(Debug)]
enum Result {
    Balanced(usize),
    Unbalanced(usize),
}

fn find_unbalanced(
    parent_child: &HashMap<String, Vec<String>>,
    weights: &HashMap<String, usize>,
    node: &str,
) -> Result {
    let weight = weights
        .get(node)
        .expect("expected every node to have a weight");

    match parent_child.get(node).unwrap_or(&Vec::new()).as_slice() {
        [] => return Result::Balanced(*weight),
        children => {
            let mut occurrences: HashMap<usize, Vec<&str>> = HashMap::new();
            for c in children {
                match find_unbalanced(parent_child, weights, c) {
                    Result::Balanced(w) => occurrences.entry(w).or_insert_with(Vec::new).push(c),
                    r => return r,
                }
            }

            match &mut occurrences.into_iter().collect::<Vec<_>>()[..] {
                [(child_weight, nodes)] => {
                    return Result::Balanced(weight + *child_weight * nodes.len());
                }
                [(child_weight1, nodes1), (child_weight2, nodes2)] => {
                    let (incorrect_weight, incorrect_weight_nodes, correct_weight) =
                        if child_weight1 > child_weight2 {
                            (child_weight1, nodes1, child_weight2)
                        } else {
                            (child_weight2, nodes2, child_weight1)
                        };

                    let node_name = incorrect_weight_nodes
                        .get(0)
                        .expect("there to be exactly one unbalanced result");
                    let weight = weights
                        .get(*node_name)
                        .expect("every node to have a weight");
                    return Result::Unbalanced(
                        weight - (incorrect_weight.abs_diff(*correct_weight)),
                    );
                }
                results => {
                    panic!("{:?} unexpected number of results", results);
                }
            }
        }
    }
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
    let nodes: Vec<&&String> = all_keys.difference(&all_children).collect();
    let root = nodes.get(0).expect("expected exactly one node");
    println!("{:?}", root);

    let result = find_unbalanced(&tree, &weights, root);
    println!("{:?}", result);
}
