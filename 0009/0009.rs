fn main() {
    const MAX: u64 = 998;
    let mut the_product: u64 = 0;

    'outer: for a in 1..MAX {
        for b in a + 1..MAX {
            for c in b + 1..MAX {
                if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000 {
                    the_product = a * b * c;
                    break 'outer;
                }
            }
        }
    }
    println!("the product is {}", the_product);
}
