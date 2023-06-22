fn main() {
    let mut product_vec: Vec<u32> = Vec::new();
    for i in 0..1000 {
        'outer: for j in 0..1000 {
            let product: u32 = i * j;
            let s: String = product.to_string();
            for s_index in 0..(s.len() / 2) {
                if s.chars().nth(s_index).unwrap() != s.chars().nth(s.len() - 1 - s_index).unwrap()
                {
                    continue 'outer;
                }
            }
            product_vec.push(product);
        }
    }
    product_vec.sort();
    println!("{:?}", product_vec);
}
