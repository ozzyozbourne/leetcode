pub fn tail_recursive_factorial(n: i32, r: i32) -> i32 {
    if n <= 1 {
        return r;
    }
    tail_recursive_factorial(n - 1, n * r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_one() {
        assert_eq!(tail_recursive_factorial(5, 1), 120);
        assert_eq!(tail_recursive_factorial(4, 1), 24);
        assert_eq!(tail_recursive_factorial(3, 1), 6);
    }
}
