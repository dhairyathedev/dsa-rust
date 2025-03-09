mod problems;
use problems::basic;
// maximum possible elements in an array in rust is 2^63 - 1
fn main() {
    basic::largest_element();
    basic::second_largest_element();
    basic::check_arr_is_sorted();
    basic::remove_duplicate_sorted();
    println!("Hello, world!");
}
