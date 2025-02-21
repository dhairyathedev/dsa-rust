//use std::mem::swap;

pub fn reverse_arr(l: usize, r: usize, vec: &mut Vec<i32>) {
    if (l>=r) {
        return;
    };
    vec.swap(l, r);
    //swap(&mut vec[l], &mut vec[r]);
    reverse_arr(l+1, r-1, vec);
}
