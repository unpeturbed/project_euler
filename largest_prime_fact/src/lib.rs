pub trait PrimeUtils {
    fn prime_factors(self) -> (Vec<u64>, u64);
}

impl PrimeUtils for u64 {
    fn prime_factors(self) -> (Vec<u64>, u64) {
        let mut prime = 2;
        let mut number = self;

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

        let mut max_prime = 0;
        for num in &lop {
            if *num > max_prime {
                max_prime = *num;
            }
        }
        (lop, max_prime)
    }
}
