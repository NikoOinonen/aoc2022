
use std::fs;
use std::str;

fn main() {
    
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path).expect("Reading contents");

    let mut calories: Vec<i32> = Vec::new();
    let mut c = 0;
    for line in str::lines(&contents) {
        match line.parse::<i32>() {
            Ok(v) => c += v,
            Err(_) => {
                calories.push(c);
                c = 0;
            }
        }
    }

    calories.sort();
    calories.reverse();
    println!("Max calories: {}", calories[0]);
    println!("Sum of top three: {}", calories.iter().take(3).sum::<i32>());

}