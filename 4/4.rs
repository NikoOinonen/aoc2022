
use std::{fs, str, collections::HashSet};

fn parse_ids(s: &str) -> Vec<Vec<i32>> {
    s.split(',')
        .map(|s_| s_.split('-')
            .map(|s__| s__.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .map(|v| (v[0]..v[1]+1).collect())
        .collect()
}

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Reading contents");

    let mut contains_count = 0;
    let mut overlap_count = 0;
    for line in str::lines(&contents) {
        let mut ids = parse_ids(line);
        let id_set1 = ids.remove(0).into_iter().collect::<HashSet<i32>>();
        let id_set2 = ids.remove(0).into_iter().collect::<HashSet<i32>>();
        if id_set1.is_subset(&id_set2) || id_set2.is_subset(&id_set1) {
            contains_count += 1;
        }
        if !id_set1.is_disjoint(&id_set2) {
            overlap_count += 1;
        }
    }
    println!("Fully contained ranges: {}", contains_count);
    println!("Overlapping ranges: {}", overlap_count);

}
