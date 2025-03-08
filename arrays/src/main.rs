mod problems;
use problems::basic;
// maximum possible elements in an array in rust is 2^63 - 1
fn main() {
    basic::largest_element();
    basic::second_largest_element();
    println!("Hello, world!");
}
