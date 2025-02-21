//use std::mem::swap;

pub fn reverse_arr(l: usize, r: usize, vec: &mut Vec<i32>) {
    if (l>=r) {
        return;
    };
    vec.swap(l, r);
    //swap(&mut vec[l], &mut vec[r]);
    reverse_arr(l+1, r-1, vec);
}


// reverse an array using single pointer approach
pub fn reverse_arr_single_ptr(i: usize, n: usize, vec: &mut Vec<i32>) {
    if i >= n / 2 {
        return;
    }
    vec.swap(i, n-i-1);
    reverse_arr_single_ptr(i+1, n, vec);
}
