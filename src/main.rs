// TODO: add another go to see if there are multiple digits
// might want to loop it
// keep adding digits together
// break if 1 digit

// since im refactoring this to a loop i probably
// dont need to call digits two or sum two

//logic loop
// num to string chars (map to digit unwrap) collect
// add digits together
// sum to string chars map digit etc...
// match len

// how do i pass the new vlaue into this loop?

fn split(num: i64) -> i64 {
    let mut sum: i64 = num;
    loop {
        // split to digits, stringify it, iter by chars, map to digits, collect as vec
        let digits: Vec<i64> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i64)
            .collect();

        // add digits
        sum = digits.iter().sum();

        let sum_digits: Vec<i64> = sum
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i64)
            .collect();
        sum = sum_digits.iter().sum();
        // return sum;
        match sum_digits.len() {
            2.. => {
                continue;
            }
            _ => {
                return sum;
            }
        }
    }
}



fn main() {
    let num = 10;
    let another = 999999999;
    let yet_another = 493193;
    let a = split(num);
    let b = split(another);
    let c = split(yet_another);
    println!("the numbers are \n{}\n{}\n{}", a, b, c)
}
