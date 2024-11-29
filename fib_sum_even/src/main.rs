use fib_sum_even::fib;

fn main() {
    let result = fib(4_000_000);
    println!("Sum of even Fibonacci terms: {}", result);
}
