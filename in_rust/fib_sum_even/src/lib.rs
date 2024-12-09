pub fn fib(limit: u32) -> u32 {
    let (mut previous, mut sum, mut current) = (0, 0, 1);

    while current <= limit {
        if current % 2 == 0 {
            sum += current;
        }
        let next = previous + current;
        previous = current;
        current = next;
    }
    sum
}
