use largest_prime_fact::PrimeUtils;
fn main() {
    let number = 600851475143;
    let (factors, largest) = number.prime_factors();
    println!("Prime factors = {:?}", factors);

    println!("The largest factor of {} is {}", number, largest);
}
