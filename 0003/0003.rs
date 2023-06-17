fn main() {
    let prime_factor_range: usize = 10000;
    let mut the_number: u64 = 600851475143;
    let prime_factor_vec: Vec<u64> = sieve_of_eratosthenes(prime_factor_range);
    let mut prime_factor_vec2: Vec<u64> = Vec::new();

    for i in 0..prime_factor_vec.len() {
        while the_number % prime_factor_vec[i] == 0 {
            the_number = the_number / prime_factor_vec[i];
            prime_factor_vec2.push(prime_factor_vec[i]);
        }
        if the_number == 1 {
            break;
        }
    }
    println!("{:?}", prime_factor_vec2);
}

fn sieve_of_eratosthenes(n: usize) -> Vec<u64> {
    let mut return_vec: Vec<u64> = Vec::new();
    let mut vec: Vec<u64> = vec![1; n];

    vec[0] = 0;
    vec[1] = 0;

    for p in 2..n {
        let mut i: usize = 2;
        while i * p < n {
            vec[i * p] = 0;
            i = i + 1;
        }
    }

    for (i, value) in vec.iter().enumerate() {
        if *value == 1 {
            return_vec.push(i as u64);
        }
    }
    return return_vec;
}
