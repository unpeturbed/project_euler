use largest_prime_fact::prime_factors;
fn main() {
    let number = 1000;
    let factors = prime_factors(number);
    println!("Prime factors = {:?}", factors);
}
