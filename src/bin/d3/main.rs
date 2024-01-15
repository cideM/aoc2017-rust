use std::io;
fn dist_to_center(n: isize) -> Option<isize> {
    let mut bottom_right = 1;
    let mut rings = 0;
    while bottom_right < n {
        bottom_right += rings * 8;
        rings += 1
    }

    // bottom, left, up, right
    let distances: [isize; 4] = [
        (n - (bottom_right - (rings - 1))).abs(),
        (n - (bottom_right - 3 * (rings - 1))).abs(),
        (n - (bottom_right - 5 * (rings - 1))).abs(),
        (n - (bottom_right - 7 * (rings - 1))).abs(),
    ];
    distances.iter().min().map(|x| x + (rings - 1))
}

// TODO: What if we extract read input into a function that returns
// ParseIntError or IoError, how can we handle the error conversions?
// https://fettblog.eu/rust-enums-wrapping-errors/

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input: isize = buffer.trim().parse().unwrap();
    match dist_to_center(input) {
        Some(d) => {
            println!("{}", d);
        }
        None => {
            println!("No value, something went wrong");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dist_to_center() {
        assert_eq!(dist_to_center(23), Some(2));
    }
}
