use std::fs;

fn main() {
    let x = std::fs::read_to_string("./inputs/input_aoc_15.txt").unwrap();
    let mut v : Vec<Vec<i64>> = vec![];
    for i in x.lines() {
        v.push(parse_part1(i));
    }
    println!("{}", max_score(&v));
}

fn parse_part1(s: &str) -> Vec<i64> {
    s.split(", ")
     .map(|x| { x.split(' ').last().unwrap().parse::<i64>().unwrap() })
     .collect::<Vec<i64>>()
}

fn max_score(matrix: &Vec<Vec<i64>>) -> i64 {
    let mut score = 0;
    let mut max = std::i64::MIN;
    for i in 1..100 {
        for j in 1..(100-i) {
            for k in 1..(100-i-j) {
                let l = 100-i-j-k;
                let a = matrix[0][0]*i+matrix[1][0]*j+matrix[2][0]*k+matrix[3][0]*l;
                let b = matrix[0][1]*i+matrix[1][1]*j+matrix[2][1]*k+matrix[3][1]*l;
                let c = matrix[0][2]*i+matrix[1][2]*j+matrix[2][2]*k+matrix[3][2]*l;
                let d = matrix[0][3]*i+matrix[1][3]*j+matrix[2][3]*k+matrix[3][3]*l;
                let e = matrix[0][4]*i+matrix[1][4]*j+matrix[2][4]*k+matrix[3][4]*l;

                if e != 500 {
                    continue;
                }

                if a <= 0 || b <= 0 || c <= 0 || d <= 0 {
                    score = 0;
                    continue;
                }
                score = a*b*c*d;
                if score > max {
                    max = score
                }
            }
        }
    }
    max
}
