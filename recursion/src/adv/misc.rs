pub fn check_palindrome(l: usize, r: usize, s: &str) -> bool {
    if l >= r {
        return true;
    }

    let bytes = s.as_bytes();
    if bytes[l] != bytes[r] {
        return false;
    }

    return check_palindrome(l+1, r-1, s);
}


pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

   return fibonacci(n-1) + fibonacci(n-2); 
}


pub fn subsequence(ind: usize, arr: &mut Vec<i32>, src: &Vec<i32>, n: usize) {
    if ind == n {
        if arr.len() == 0 {
            println!("null");
            return;
        }
        println!("{:?}", arr);
        return;
    }
    // take or pick the particular index into the subsequence
    arr.push(src[ind]);
    subsequence(ind+1, arr, src, n);
    arr.pop();

    // not pick, or not take condition, this element is not added to your subsequence
    subsequence(ind+1, arr, src, n);
}
