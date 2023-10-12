fn main() {
    let mut sum_of_squares: i32 = 0;
    for i in 1..101 {
        sum_of_squares += (i as i32).pow(2);
    }
    let square_of_sums: i32 = 5050_i32.pow(2);
    println!("{}", square_of_sums - sum_of_squares);
}
