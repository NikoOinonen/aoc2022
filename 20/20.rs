
use std::fs;

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();
    let nums: Vec<(usize, i32)> = input.trim().split("\n").map(|s| s.trim().parse().unwrap()).enumerate().collect();

    {
        let mut nums = nums.clone();
        let cycle_len = (nums.len() as i32) - 1;
        for i in 0..nums.len() {
            let ind = nums.iter().position(|(ind, _)| *ind == i).unwrap();
            let num = nums.remove(ind);
            let new_ind = (ind as i32) + num.1;
            let new_ind_mod = (((new_ind % cycle_len) + cycle_len) % cycle_len) as usize;
            nums.insert(new_ind_mod as usize, num);
        }

        let zero_pos = nums.iter().position(|(_, v)| *v == 0).unwrap();
        let mut sum_of_grove_coords = 0;
        for pos in [1000, 2000, 3000] {
            sum_of_grove_coords += nums[(pos + zero_pos) % nums.len()].1;
        }
        println!("Sum of grove coordinates (1 round): {sum_of_grove_coords}");
    }

    {
        let mut nums: Vec<(usize, i64)> = nums.iter().map(|(ind, v)| (*ind, 811589153 * (*v as i64))).collect();
        let cycle_len = (nums.len() as i64) - 1;
        for _ in 0..10 {
            for i in 0..nums.len() {
                let ind = nums.iter().position(|(ind, _)| *ind == i).unwrap();
                let num = nums.remove(ind);
                let new_ind = (ind as i64) + num.1;
                let new_ind_mod = (((new_ind % cycle_len) + cycle_len) % cycle_len) as usize;
                nums.insert(new_ind_mod as usize, num);
            }
        }

        let zero_pos = nums.iter().position(|(_, v)| *v == 0).unwrap();
        let mut sum_of_grove_coords = 0;
        for pos in [1000, 2000, 3000] {
            sum_of_grove_coords += nums[(pos + zero_pos) % nums.len()].1;
        }
        println!("Sum of grove coordinates (10 rounds): {sum_of_grove_coords}");
    }

}
