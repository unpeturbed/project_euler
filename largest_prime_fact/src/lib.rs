pub fn prime_factors(mut number: u64) -> Vec<u64> {
    let mut prime = 2;

    let mut lop: Vec<u64> = Vec::new();

    while prime * prime <= number {
        while number % prime == 0 {
            lop.push(prime);
            number = number / prime;
        }
        prime = prime + 1;
    }
    if number > 1 {
        lop.push(number);
    }
    lop
}
