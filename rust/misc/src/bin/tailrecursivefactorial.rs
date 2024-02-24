pub fn tail_recursive_factorial(n: i32, r: i32) -> i32 {
    if n <= 1 {
        return r;
    }
    tail_recursive_factorial(n - 1, n * r)
}

fn main() {
    println!("{}", tail_recursive_factorial(4, 1));
}
