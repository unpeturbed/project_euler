pub fn mult(x: i32, n: i32) -> i32 {
    let mut multiples = Vec::new();
    let numbers = x..n;

    for number in numbers {
        if number % 3 == 0 || number % 5 == 0 {
            multiples.push(number);
        }
    }
    let mut sum = 0;
    for multiple in &multiples {
        sum = sum + multiple;
    }
    sum
}
