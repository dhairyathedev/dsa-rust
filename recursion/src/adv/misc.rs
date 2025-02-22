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
