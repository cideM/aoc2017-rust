use std::collections::VecDeque;
use std::io;

// TODO: Can we make this work for ring 1 and 2?
fn make_tile_sequence(ring_number: usize) -> VecDeque<Vec<usize>> {
    let current_ring_size = (ring_number - 1) * 8;
    // TODO: Better name than just n?
    let n = current_ring_size / 4;
    let edge_lengths: [usize; 4] = [n - 4, n - 3, n - 3, n - 2];

    let mut m = current_ring_size - 8;

    let mut ring = VecDeque::new();
    for i in 0..4 {
        ring.push_back(vec![1, m]);
        ring.push_back(vec![1, 2, m, m + 1]);
        for _ in 0..edge_lengths[i] {
            ring.push_back(vec![1, m, m + 1, m + 2]);
        }
        ring.push_back(vec![1, m + 1, m + 2]);
        m += 2;
    }

    return ring;
}

struct SpiralIterator {
    sequence: Vec<usize>,
    steps: VecDeque<Vec<usize>>,
    ring: usize,
    pos: usize,
}

impl Iterator for SpiralIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.sequence.len() {
            // need to actually generate a number now
            if self.steps.len() == 0 {
                self.steps = make_tile_sequence(self.ring);
                self.ring += 1;
            }

            // TODO: tuples?
            let step = self.steps.pop_front().unwrap();
            let mut sum = 0;
            for n in step.iter() {
                sum += self.sequence[self.pos - *n];
            }
            self.sequence.push(sum);
        }

        let current = self.sequence[self.pos];

        self.pos += 1;

        Some(current)
    }
}

fn build_spiral_iterator() -> SpiralIterator {
    SpiralIterator {
        sequence: vec![1, 1, 2, 4, 5, 10, 11, 23, 25],
        pos: 0,
        ring: 3,
        steps: VecDeque::new(),
    }
}

fn dist_to_center(n: usize) -> Option<usize> {
    let mut bottom_right = 1;
    let mut rings = 0;
    while bottom_right < n {
        bottom_right += rings * 8;
        rings += 1
    }

    // bottom, left, up, right
    let distances: [usize; 4] = [
        n.abs_diff(bottom_right - (rings - 1)),
        n.abs_diff(bottom_right - 3 * (rings - 1)),
        n.abs_diff(bottom_right - 5 * (rings - 1)),
        n.abs_diff(bottom_right - 7 * (rings - 1)),
    ];
    distances.iter().min().map(|x| x + (rings - 1))
}

// TODO: What if we extract read input into a function that returns
// ParseIntError or IoError, how can we handle the error conversions?
// https://fettblog.eu/rust-enums-wrapping-errors/

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let input: usize = buffer.trim().parse().unwrap();
    match dist_to_center(input) {
        Some(d) => {
            println!("{}", d);
        }
        None => {
            println!("No value, something went wrong");
            std::process::exit(1);
        }
    }

    let mut spiral_iter = build_spiral_iterator();
    while let Some(i) = spiral_iter.next() {
        if i > input {
            println!("{}", i);
            std::process::exit(0);
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

    #[test]
    fn spiral_iterator() {
        let mut spiral_iter = build_spiral_iterator();
        assert_eq!(spiral_iter.next(), Some(1));
        assert_eq!(spiral_iter.next(), Some(1));
        assert_eq!(spiral_iter.next(), Some(2));
    }

    #[test]
    fn spiral_iterator_third_ring() {
        let spiral_iter = build_spiral_iterator();
        let values = spiral_iter.take(25).collect::<Vec<usize>>();
        assert_eq!(
            values,
            vec![
                1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351,
                362, 747, 806, 880, 931,
            ]
        );
    }

    #[test]
    fn spiral_iterator_fourth_ring() {
        let spiral_iter = build_spiral_iterator();
        let values = spiral_iter.take(26).collect::<Vec<usize>>();
        assert_eq!(
            values,
            vec![
                1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351,
                362, 747, 806, 880, 931, 957
            ]
        );
    }
}
