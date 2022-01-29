use std::fs;

fn main() {
    let content: Vec<char> = fs::read_to_string("./inputs/input_aoc_8.txt")
        .unwrap()
        .chars()
        .collect();

    println!("part1 : {}", part1(&content));
    println!("part2 : {}", part2(&content));
}

fn part1(content : &Vec<char>) -> usize {
    let mut i = 0;
    let mut n = 0;
    while i < content.len() {
        if content[i] == '"' || content[i] == ' ' {
            i += 1;
            continue;
        } else if content[i] == '\\' { // escape sequence
            i+=1;
            if content[i] == '\\' || content[i] == '"' {
                n+=1;
                i+=1;
                continue;
            } else if content[i] == 'x' {
                n+=1;
                i+=3;
                continue;
            } else {
                panic!("Unknown character at position {} ({})", i, content[i]);
            }
        }
        n+=1;
        i+=1;
    }
    content.len() - n
}

fn part2(content : &Vec<char>) -> usize {
    let mut i = 0;
    let mut n = 0;
    while i < content.len() {
        if content[i] == '"'  || content[i] == '\\' {
            n+=2;
        } else if content[i] == '\n' {
            n+=3;
        } else {
            n+=1;
        }
        i+=1;
    }
    n - content.len()
}
