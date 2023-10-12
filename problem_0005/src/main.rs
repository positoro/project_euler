fn main() {
    const MAX_NUMBER: u32 = 1000000000;

    for i in 1..MAX_NUMBER {
        if check(i) == true {
            println!("{} is evenly divisible.", i);
            break;
        }
    }
}

fn check(n: u32) -> bool {
    for i in 1..21 {
        if n % i != 0 {
            return false;
        }
    }

    return true;
}
