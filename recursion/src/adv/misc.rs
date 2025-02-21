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
