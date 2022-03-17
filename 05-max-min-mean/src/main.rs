use std::cmp;

fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    
    let mut max: i32 = numbers[0];
    let mut min: i32 = numbers[0];
    let mut total = 0.0;

    for number in numbers {
        max = cmp::max(max, number);
        min = cmp::min(min, number);
        total += number as f64;
    }
    let mean = total / numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
