use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n = input.trim().parse::<i32>().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().filter_map(|s| s.trim().parse::<i32>().ok()).collect();

    // precompute
    let mut hash: HashMap<i32, i32> = HashMap::new();
    for &n in &nums {
        *hash.entry(n).or_insert(0) += 1;
    }
    println!("{:?}", hash);
    println!("Hello, world!");
}
