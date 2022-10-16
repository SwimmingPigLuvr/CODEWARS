fn digital_root(num: i64) -> i64 {

    // hold our num as sum
    let mut sum: i64 = num;
    loop {
        // create vec to hold digits
        // i64 to string
        // iterate by chars
        // map each char to digit (base 10 number)
        // unwrap as i64
        // collect into vector
        let digits: Vec<i64> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i64)
            .collect();

        // add digits
        sum = digits.iter().sum();

        // repeat process of splitting the digits for the sum
        let sum_digits: Vec<i64> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i64)
            .collect();

        // add sum digits
        sum = sum_digits.iter().sum();


        // match number of digits
        match sum_digits.len() {
        // if there are more than 1 digits => the loop repeats
            2.. => {
                continue;
            }
        // if there is only one digit => we return sum
            _ => {
                return sum;
            }
        }
    }
}



fn main() {
    let one = 22;
    let two = 493192;
    let three = 999999999;
    let a = digital_root(one);
    let b = digital_root(two);
    let c = digital_root(three);
    println!("the numbers are {}{}{}", a, b, c)
}
