fn main() {
    let prime_factor_range: usize = 10000;
    let mut the_number: u64 = 600851475143;
    let the_prime_factor_vec: Vec<u64> = sieve_of_eratosthenes(prime_factor_range);
    let mut prime_factor_in_the_number: Vec<u64> = Vec::new();

    for i in 0..the_prime_factor_vec.len() {
        while the_number % the_prime_factor_vec[i] == 0 {
            the_number = the_number / the_prime_factor_vec[i];
            prime_factor_in_the_number.push(the_prime_factor_vec[i]);
        }
        if the_number == 1 {
            break;
        }
    }
    println!("{:?}", prime_factor_in_the_number);
}

fn sieve_of_eratosthenes(n: usize) -> Vec<u64> {
    let mut return_vec: Vec<u64> = Vec::new();
    let mut vec: Vec<bool> = vec![true; n];

    vec[0] = false;
    vec[1] = false;

    for i in 2..(n as f64).sqrt() as usize {
        for j in 2..(n as f64).sqrt() as usize {
            vec[i * j] = false;
        }
    }

    for (i, value) in vec.iter().enumerate() {
        if *value == true {
            return_vec.push(i as u64);
        }
    }
    return return_vec;
}
