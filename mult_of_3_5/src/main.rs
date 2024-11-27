//If we list all the natural numbers below 10
//that are multiples of 3 or 5, we get 3,5,6 and 9
//The sum of these multiples is 23
//Find the sum of all the multiples of 3 or 5 below 1000
use mult_of_3_5::mult;
fn main() {
    let sum = mult(1, 1000);
    println!("{sum}");
}
