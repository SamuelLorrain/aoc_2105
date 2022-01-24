use std::fs;

fn main() {
    let content = fs::read_to_string("input_aoc_1.txt").unwrap();
    println!("{}", find_floor(&content));
    println!("{}", find_minus_one(&content));
}

fn find_floor(s: &str) -> i64 {
    let mut i : i64 = 0;

    for c in s.chars() {
        if c == '(' { i += 1; }
        else if c == ')' { i -= 1 };
    }
    i
}

fn find_minus_one(s: &str) -> i64 {
    let mut n : i64 = 0;
    let mut i : i64 = 0;

    for c in s.chars() {
        i+=1;
        if c == '(' { n += 1; }
        else if c == ')' { n -= 1 };
        if n == -1 { return i; }
    }
    n
}
