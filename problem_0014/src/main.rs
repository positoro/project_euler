fn main() {
    const UPPER: u32 = 1000000;
    let mut max_counts: u128 = 0;

    for i in 0..UPPER {
        let counts: u128 = get_chain_counts(i);
        if max_counts < counts {
            max_counts = counts;
        }
    }
    println!("{:?}", max_counts);
}

fn get_chain_counts(mut s: u32) -> u128 {
    let mut return_counts: u128 = 0;
    s = 159487;

    while s > 1 {
        return_counts = return_counts + 1;
        if s % 2 == 0 {
            s = s / 2;
        } else {
            s = s * 3 + 1;
            println!("{:?}", s);
        }
    }

    return return_counts;
}
