use std::collections::HashMap;
use std::io;
use std::process;

fn main() {
    let mut registers: HashMap<String, isize> = HashMap::new();
    for maybe_line in io::stdin().lines() {
        let line = maybe_line.expect("line should be Ok()");
        let parts = line.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            &[var, op, amount_, _, a, cmp, b_] => {
                let amount_value: isize = amount_.parse().expect("amount should be number");
                let b_value: isize = b_.parse().expect("b should be number");
                let a_value = registers.get(a).unwrap_or(&0);
                let pass = match cmp {
                    ">" => a_value > &b_value,
                    "<" => a_value < &b_value,
                    "!=" => a_value != &b_value,
                    "==" => a_value == &b_value,
                    ">=" => a_value >= &b_value,
                    "<=" => a_value <= &b_value,
                    x => {
                        println!("expected comparator but got {:?}", x);
                        process::exit(1);
                    }
                };
                if pass {
                    let entry = registers.entry(var.to_owned()).or_insert(0);
                    match op {
                        "inc" => {
                            *entry += amount_value;
                        }
                        "dec" => {
                            *entry -= amount_value;
                        }
                        x => {
                            println!("expected inc or dec but got {:?}", x);
                            process::exit(1);
                        }
                    }
                }
            }
            x => {
                println!("{:?}", x);
                process::exit(1);
            }
        }
    }
	let largest_value = registers.values().max();
    println!("{:?}", largest_value);
}
