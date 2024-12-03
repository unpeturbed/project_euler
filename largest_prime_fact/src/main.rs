use largest_prime_fact::prime_factors;
fn main() {
    let number = 600851475143;
    let (factors, largest) = prime_factors(number);
    println!("Prime factors = {:?}", factors);

    println!("The largest factor of {} is {}", number, largest);
}
