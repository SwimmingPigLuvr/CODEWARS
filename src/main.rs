// TODO: add another go to see if there are multiple digits
// might want to loop it
// keep adding digits together
// break if 1 digit




fn split(num: i64) -> i64 {
    let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
    // add digits
    let mut sum: i64 = 0;
    for d in digits {
        sum += d
    }
    // if there are still two digits then we stringify them and add them together again
    let digits_two: Vec<_> = sum.to_string().chars().map(|d| d.to_digit(10).unwrap() as i64).collect();
    match digits_two.len() {
        x if x > 1 => {
            let mut sum_two = 0;
            for d in digits_two {
                sum_two += d
            }
            sum_two
        }
        _ => {sum}
    }
    
    
        
    }


fn main() {
    let num = 10;
    let another = 999999999;
    let yet_another = 493193;
    let a = split(num);
    let b = split(another);
    let c = split(yet_another);
    println!("{}\n{}\n{}", a, b, c)
}
