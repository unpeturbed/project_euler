fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn smallest_multiple(limit: u64) -> u64 {
    (1..=limit).fold(1, |acc, x| lcm(acc, x))
}

fn main() {
    let result = smallest_multiple(20);
    println!(
        "The smallest number divisible by all numbers from 1 to 20 is: {}",
        result
    );
}
