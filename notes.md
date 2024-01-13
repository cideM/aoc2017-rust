## Day 2

There's nothing you can do with `nums` in this snippet because `line.unwrap()` goes out of scope?

```rust
use std::io;

fn main() {
    for line in io::stdin().lines() {
        let nums = line.unwrap().split(' ').map(|x| x.parse::<u32>().unwrap());
        println!("{:?}", nums)
    }
}
```
