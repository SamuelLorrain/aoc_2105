use std::fs;

const SIZE: usize = 100;

fn main() {
    let content = fs::read_to_string("inputs/input_aoc_18.txt").unwrap();
    let mut map = parse(&content);

    for _ in 0..100 {
        map = next_step(&map);
    }
    println!("{}", count_light_on(&map));
}

fn parse(s:&str) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = vec![vec![false; SIZE]; SIZE];
    for (index_l,line) in s.lines().enumerate() {
        for (index_c,ch) in line.chars().enumerate() {
            if ch == '.' {
                map[index_l][index_c] = false;
            }
            if ch == '#' {
                map[index_l][index_c] = true;
            }
        }
    }
    map
}

fn next_step(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut neighbours : Vec<bool> = vec![false; 8];
    let mut new_map: Vec<Vec<bool>> = vec![vec![false; SIZE]; SIZE];
    for (index_i, i) in map.iter().enumerate() {
        for (index_j, _) in i.iter().enumerate() {
            if index_j < SIZE-1 {
                neighbours[0] = map[index_i][index_j+1];
            }
            if index_j < SIZE-1 && index_i < SIZE-1 {
                neighbours[1] = map[index_i+1][index_j+1];
            }
            if index_i < SIZE-1 {
                neighbours[2] = map[index_i+1][index_j];
            }
            if index_i < SIZE-1 && index_j > 0 {
                neighbours[3] = map[index_i+1][index_j-1];
            }
            if index_j > 0 {
                neighbours[4] = map[index_i][index_j-1];
            }
            if index_i > 0 && index_j > 0 {
                neighbours[5] = map[index_i-1][index_j-1];
            }
            if index_i > 0 {
                neighbours[6] = map[index_i-1][index_j];
            }
            if index_i > 0 && index_j < SIZE-1 {
                neighbours[7] = map[index_i-1][index_j+1];
            }
            let mut n = 0;
            for i in neighbours.iter() {
                if *i == true {
                    n+=1;
                }
            }
            if map[index_i][index_j] && n == 2 || n == 3 {
                new_map[index_i][index_j] = true;
            } else if !map[index_i][index_j] && n == 3 {
                new_map[index_i][index_j] = true;
            } else {
                new_map[index_i][index_j] = false;
            }

            // part 2
            new_map[0][0] = true;
            new_map[0][SIZE-1] = true;
            new_map[SIZE-1][0] = true;
            new_map[SIZE-1][SIZE-1] = true;

            for i in neighbours.iter_mut() {
                *i = false;
            }
        }
    }
    new_map
}

fn count_light_on(map: &Vec<Vec<bool>>) -> i64 {
    let mut n = 0;
    for i in map.iter() {
        for j in i.iter() {
            if *j == true {
                n+=1;
            }
        }
    }
    n
}
