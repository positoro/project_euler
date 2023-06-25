fn main() {
    let mut array_for_the_fibonacci_sequence: [u32; 3] = [0; 3];

    const FIRST_SEQUENCE: u32 = 1;
    const SECOND_SEQUENCE: u32 = 2;

    let mut sum: u32 = 0;
    let mut last_sequence: u32 = 0;

    array_for_the_fibonacci_sequence[0] = FIRST_SEQUENCE;
    check_even_and_add(FIRST_SEQUENCE, &mut sum);
    array_for_the_fibonacci_sequence[1] = SECOND_SEQUENCE;
    check_even_and_add(SECOND_SEQUENCE, &mut sum);

    while last_sequence <= 4000000 {
        last_sequence = proceed_fibonacci_sequence(&mut array_for_the_fibonacci_sequence);
        check_even_and_add(last_sequence, &mut sum);
    }

    println!("sum: {}", sum);
}

fn check_even_and_add(n: u32, sum: &mut u32) {
    if n % 2 == 0 {
        *sum += n;
    }
}

fn proceed_fibonacci_sequence(array: &mut [u32; 3]) -> u32 {
    array[2] = array[0] + array[1];
    array[0] = array[1];
    array[1] = array[2];

    return array[2];
}
