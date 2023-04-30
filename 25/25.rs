use std::fs;

fn snafu_to_decimal(snafu_str: &str) -> i64 {
    let mut num = 0;
    for (e, c) in snafu_str.chars().rev().enumerate() {
        num += 5_i64.pow(e as u32) * match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Invalid character {c}")
        };
    }
    num
}

fn decimal_to_snafu(num: i64) -> String {

    fn recurse(mut num: i64, snafu_digits: &mut Vec<i64>) {
        if num == 0 {
            return;
        }
        let mut minus = 0;
        let mut place = 0;
        loop {
            let new_minus = minus + 2 * 5_i64.pow(place);
            if new_minus < num.abs() {
                minus = new_minus;
            } else {
                break;
            }
            place += 1;
        }
        let base = 5_i64.pow(place);
        let digit = (num + minus * num.signum()) / base;
        if snafu_digits.len() == 0 {
            for _ in 0..place {
                snafu_digits.push(0);
            }
            snafu_digits.push(digit);
        } else {
            snafu_digits[place as usize] = digit;
        }
        num -= digit * base;
        recurse(num, snafu_digits);
    }

    let mut snafu_digits: Vec<i64> = Vec::new();
    recurse(num, &mut snafu_digits);

    let mut snafu = String::new();
    for digit in snafu_digits.iter().rev() {
        snafu.push(
            match digit {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!("Invalid digit {digit}")
            }
        );
    }
    
    snafu

}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

//     let input = "1=-0-2
// 12111
// 2=0=
// 21
// 2=01
// 111
// 20012
// 112
// 1=-1=
// 1-12
// 12
// 1=
// 122".to_owned();

    let nums: Vec<i64> = input.lines().map(|line| snafu_to_decimal(line.trim())).collect();
    let sum: i64 = nums.iter().sum();

    let bob_snafu = decimal_to_snafu(sum);
    println!("{sum:?} => {bob_snafu}");

}
