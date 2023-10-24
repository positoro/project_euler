fn main() {
    const UPPER: u128 = 1000000;
    let mut max_counts: u32 = 0;
    let mut max_index: u128 = 0;

    for i in 1..UPPER {
        let counts: u32 = get_chain_terms(i);
        if max_counts < counts {
            max_counts = counts;
            max_index = i;
        }
    }
    println!("{:?} contains {:?} terms.", max_index, max_counts);
}

fn get_chain_terms(mut s: u128) -> u32 {
    let mut return_terms: u32 = 1;

    while s > 1 {
        return_terms = return_terms + 1;
        if s % 2 == 0 {
            s = s / 2;
        } else {
            s = s * 3 + 1;
        }
    }

    return return_terms;
}
