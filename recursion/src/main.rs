#![allow(unused, dead_code)]

mod adv;
use adv::{misc};

fn backtrack_1_to_n(i: i32) {
    if i < 1 {return};
    backtrack_1_to_n(i-1);
    println!("{i}");
}

fn backtrack_n_to_1(i: i32, n: i32) {
    if i > n {
        return 
    };

    backtrack_n_to_1(i+1, n);
    println!("{i}");
}

fn sum_1_to_n_param(i: i32, sum: i32) {
    if i < 1 {
        println!("{sum}");
        return;
    }

    sum_1_to_n_param(i-1, sum+i);
}

fn sum_1_to_n_func(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    return n + sum_1_to_n_func(n-1);
}

// Task: Factorial of the number
fn factorial(n: i32) -> i32 {
    if n == 0 || n == 1{
        return 1;
    }
    return n * factorial(n-1);
}


fn main() {
    //backtrack_1_to_n(3);
    //backtrack_n_to_1(0, 3);
    sum_1_to_n_param(3, 0);
    let sum = sum_1_to_n_func(3);
    println!("{sum}");
    let mut vec = vec![2, 7, 9, 2];
    adv::arr::reverse_arr(0, vec.len()-1, &mut vec);
    println!("{:?}", vec);
    let factorial_ = factorial(5);
    println!("{}", factorial_);

    let string = String::from("google");
    println!("Palindrome status: {}", misc::check_palindrome(0, string.len()-1, &string));
    println!("Hello, world!");
}
