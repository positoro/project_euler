fn main() {
    let mut sum: u32 = 0;
    let mut the_fibonacci_sequence_vec: Vec<u32> = Vec::new();

    const first_sequence = 1;
    const second_sequence = 2;

    if add_last_the_fibonacci_sequence_vec_and_is_last_even(first_sequence, &the_fibonacci_sequence_vec) {
        sum += first_sequence;
    }

    if add_last_the_fibonacci_sequence_vec_and_is_last_even(second_sequence, &the_fibonacci_sequence_vec) {
        sum += second_sequence;
    }
    let mut last_sequence= first_sequence + second_sequence;

    while last_sequence <= 4000000 {
    if add_last_the_fibonacci_sequence_vec_and_is_last_even(last_sequence, &the_fibonacci_sequence_vec) {
        sum += last_sequence;
    }
    }

    println!("sum: {}", sum);
}

fn add_last_the_fibonacci_sequence_vec_and_is_last_even(
    n: u32,
    the_fibonacci_sequence_vec: &Vec<u32>,
) -> bool {
}
