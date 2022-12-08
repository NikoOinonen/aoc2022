
use std::{fs, str};

fn main() {

    let input = fs::read_to_string("input.txt").expect("Reading input");

    let mut trees: Vec<Vec<i32>> = Vec::new();
    let mut n_cols = 0;
    let mut n_rows = 0;
    for line in str::lines(&input) {
        if line.trim().len() == 0 {continue;}
        let mut heights = Vec::new();
        for t in line.trim().chars() {
            if n_rows == 0 {
                n_cols += 1;
            }
            heights.push(t.to_digit(10).unwrap() as i32);
        }
        trees.push(heights);
        n_rows += 1;
    }

    let mut visible: Vec<Vec<bool>> = vec![vec![false; n_cols]; n_rows];
    for i in 0..n_rows {
        let mut tallest_left = -1;
        let mut tallest_right = -1;
        for j in 0..n_cols {
            if trees[i][j] > tallest_left {
                tallest_left = trees[i][j];
                visible[i][j] = true;
            }
            if trees[i][n_cols - j - 1] > tallest_right {
                tallest_right = trees[i][n_cols - j - 1];
                visible[i][n_cols - j - 1] = true;
            }
        }
    }
    for j in 0..n_cols {
        let mut tallest_top = -1;
        let mut tallest_bottom = -1;
        for i in 0..n_rows {
            if trees[i][j] > tallest_top {
                tallest_top = trees[i][j];
                visible[i][j] = true;
            }
            if trees[n_rows - i - 1][j] > tallest_bottom {
                tallest_bottom = trees[n_rows - i - 1][j];
                visible[n_rows - i - 1][j] = true;
            }
        }
    }

    let mut n_visible = 0;
    for row in visible.iter() {
        for v in row.iter() {
            if *v {
                n_visible += 1;
            }
        }
    }
    println!("Number of trees visible: {n_visible}");

    let mut best_score = 0;
    
    for i in 1..n_rows-1 {
        for j in 1..n_cols-1 {
            let h = trees[i][j];
            let mut score_rd = 1;
            let mut score_ru = 1;
            let mut score_cr = 1;
            let mut score_cl = 1;
            for ii in i+1..n_rows-1 {
                if trees[ii][j] >= h {break;}
                score_rd += 1;
            }
            for ii in (1..i).rev() {
                if trees[ii][j] >= h {break;}
                score_ru += 1;
            }
            for jj in j+1..n_cols-1 {
                if trees[i][jj] >= h {break;}
                score_cr += 1;
            }
            for jj in (1..j).rev() {
                if trees[i][jj] >= h {break;}
                score_cl += 1;
            }
            let score = score_rd * score_ru * score_cr * score_cl;
            if score > best_score {
                best_score = score;
            }
        }
    }
    println!("Best scenic score: {}", best_score);

}
