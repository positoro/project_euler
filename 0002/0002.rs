fn main() {
    let mut sum: u32 = 2;
    let mut a: u32 = 1;
    let mut b: u32 = 2;
    let mut c: u32 = 0;
    while c <= 4000000 {
        c = a + b;
        if c % 2 == 0 {
            sum += c;
        }
        a = b;
        b = c;
    }

    println!("sum: {}", sum);
}
