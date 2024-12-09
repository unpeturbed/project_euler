//By considering the terms in the Fibonacci sequence whose values do not exceed four million,
//find the sum of the even-valued terms.
use fib_sum_even::fib;

fn main() {
    let result = fib(4_000_000);
    println!("Sum of even Fibonacci terms: {}", result);
}
