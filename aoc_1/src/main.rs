use std::time::Instant;
use std::{env, fs};

fn process(input: String) -> u32 {
    let c_iter = input.chars();
    let mut digits: Vec<u32> = Vec::with_capacity(2);
    let mut sum: u32 = 0;

    for c in c_iter {
        if c == 0xA as char {
            sum += digits[0] * 10 + digits[1];
            digits.clear();
        }

        if c.is_ascii_digit() && digits.is_empty() {
            digits.push(c.to_digit(10).unwrap());
            digits.push(c.to_digit(10).unwrap());
        }
        if c.is_ascii_digit() {
            digits[1] = c.to_digit(10).unwrap();
        }
    }
    sum += digits[0] * 10 + digits[1];
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let input_txt: String = fs::read_to_string(file_path).unwrap();
    let start_time = Instant::now();
    let sum = process(input_txt);
    let end_time = Instant::now();
    println!("sum {}", sum);

    // Calculate the elapsed time
    let elapsed_time = end_time - start_time;

    println!("Elapsed time: {:?}", elapsed_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, process(input.to_string()));
    }
}
