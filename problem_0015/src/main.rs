fn main() {
    println!("There are {:?} routes.", calculate_combination(40, 20));
}

fn calculate_combination(n: u128, r: u128) -> u128 {
    let mut return_value: u128 = 1;
    for i in (n - r + 1)..(n + 1) {
        return_value = return_value * i;
    }
    return_value = return_value / calculate_factorial(r);
    return return_value;
}

fn calculate_factorial(n: u128) -> u128 {
    let mut return_value: u128 = 1;
    for i in 1..n + 1 {
        return_value = return_value * i;
    }
    return return_value;
}
