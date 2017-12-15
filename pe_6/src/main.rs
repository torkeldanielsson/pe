fn main() {
    let mut sum_of_squares: u64 = 0;
    let mut sum: u64 = 0;

    for n in 0..101 {
        sum_of_squares += n*n;
        sum += n;
    }

    println!("sum: {}", sum);
    println!("sum_of_squares: {}", sum_of_squares);
    println!("square of sum: {}", sum*sum);
    println!("diff: {}", sum*sum - sum_of_squares);
}
