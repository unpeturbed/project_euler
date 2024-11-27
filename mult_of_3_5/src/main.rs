//If we list all the natural numbers below 10
//that are multiples of 3 or 5, we get 3,5,6 and 9
//The sum of these multiples is 23
//Find the sum of all the multiples of 3 or 5 below 1000

fn main() {
    let numbers = 1..1000;
    let mut multiples = Vec::new();

    for number in numbers {
        if number % 3 == 0 || number % 5 == 0 {
            multiples.push(number);
        }
    }
    let mut sum = 0;
    for multiple in &multiples {
        sum = sum + multiple;
    }
    println!("{sum}");
}
