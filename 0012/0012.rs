fn main() {
    const TRIANGLE_NUMBERS_SIZE: usize = 12376;
    let mut the_triangle_number: u64 = 0;
    let triangle_numbers_vec: Vec<u64> = create_triangle_numbers_vec(TRIANGLE_NUMBERS_SIZE);

    for i in 0..TRIANGLE_NUMBERS_SIZE {
        let divisors_vec: Vec<u64> = get_divisors_vec(triangle_numbers_vec[i]);

        if divisors_vec.len() > 500 {
            the_triangle_number = triangle_numbers_vec[i];
            break;
        }
    }

    println!(
        "The value of the first triangle number to have over five hundred divisors is {:?}.",
        the_triangle_number
    );
}

fn create_triangle_numbers_vec(n: usize) -> Vec<u64> {
    let mut return_vec: Vec<u64> = Vec::new();
    let mut triangle_number: u64 = 0;

    for i in 0..n {
        triangle_number = triangle_number + i as u64;
        return_vec.push(triangle_number);
    }
    return return_vec;
}

fn get_divisors_vec(n: u64) -> Vec<u64> {
    let mut return_vec: Vec<u64> = Vec::new();
    let max_index: u64 = (n as f64).sqrt() as u64 + 1;

    for i in 1..max_index {
        if n % i == 0 {
            return_vec.push(i);
            if n / i != i {
                return_vec.push(n / i);
            }
        }
    }

    return return_vec;
}
