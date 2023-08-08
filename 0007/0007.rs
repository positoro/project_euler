fn main() {
    let the_prime_number_vec: Vec<u64> = create_prime_number_vec(104744);

    println!("10001th prime number is {}", the_prime_number_vec[10000]);
}

fn create_prime_number_vec(n: usize) -> Vec<u64> {
    let mut return_vec: Vec<u64> = Vec::new();
    let mut vec: Vec<bool> = vec![true; n];
    vec[0] = false;
    vec[1] = false;

    for i in 2..n {
        for j in 2..((i as f64).sqrt() + 1.0) as usize {
            if i % j == 0 {
                vec[i] = false;
            };
        }
    }

    for (i, value) in vec.iter().enumerate() {
        if *value == true {
            return_vec.push(i as u64);
        }
    }
    return return_vec;
}
