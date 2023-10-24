fn main() {
    let mut sum: String = "0".to_string();

    sum = add_two_strings(&mut sum, &mut s.clone());

    println!("{:?}", sum);
}

fn multiplication_two_strings(s1: &mut String, s2: &mut String) -> String {
    let mut return_string = String::new();

    return return_string;
}

fn multiplication_strings_and_digit(s1: &mut String, d: u8) -> String {
    let mut return_string = String::new();

    let c1: char = s1.pop().unwrap_or('0');
    return return_string;
}


fn add_two_strings(s1: &mut String, s2: &mut String) -> String {
    let mut return_string = String::new();
    let mut digit_flag: bool = false;

    while s1.is_empty() == false || s2.is_empty() == false {
        let c1: char = s1.pop().unwrap_or('0');
        let c2: char = s2.pop().unwrap_or('0');
        let mut sum: u8 = c1 as u8 - 48 + c2 as u8 - 48;
        if digit_flag == true {
            sum = sum + 1;
        }
        if sum >= 10 {
            digit_flag = true
        } else {
            digit_flag = false;
        }

        return_string.insert(
            0,
            std::char::from_digit((sum % 10 as u8).into(), 10).unwrap(),
        );
    }

    if digit_flag == true {
        return_string.insert(0, '1');
    }

    return return_string;
}
