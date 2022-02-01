use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let s = std::fs::read_to_string("./src/inputs/input_aoc_13.txt").unwrap();
    let mut scores: Vec<(String,String,i64)> = vec![];
    let part_2 = true;
    for i in s.lines() {
        scores.push(parse(i));
    }
    if part_2 == true {
        for i in guests_list(&scores) {
            scores.push(parse(format!("I would gain 0 happiness units by sitting next to {}.", i).as_str()));
            scores.push(parse(format!("{} would gain 0 happiness units by sitting next to I.", i).as_str()));
        }
    }

    let guests = guests_list(&scores).into_iter().permutations(guests_list(&scores).len());
    let mut max = std::i64::MIN;
    for i in guests {
        let current_score = score(&i, &scores);
        if max < current_score {
            max = current_score;
        }
    }
    println!("{}", max);
}

fn parse(s: &str) -> (String, String, i64) {
    let v: Vec<&str> = s.trim_end_matches('.').split(' ').collect();
    let a: String = v[0].to_string();
    let b: String = v.last().unwrap().to_string();
    let mut c: i64 = v[3].parse::<i64>().unwrap();
    if v[2] == "lose" {
        c*=-1;
    }
    (a,b,c)
}

fn guests_list(v: &Vec<(String, String, i64)>) -> Vec<String> {
    let mut h = HashSet::new();
    for (i,j,_) in v {
        h.insert(i.clone());
        h.insert(j.clone());
    }
    h.iter().map(|x| {x.clone()}).collect()
}

fn score(guest_arrangement: &Vec<String>, scores: &Vec<(String,String,i64)>) -> i64 {
    let mut i = 0;
    let mut n = 0;
    while i < (guest_arrangement.len()-1) {
        let a = guest_arrangement[i].to_string();
        let b = guest_arrangement[i+1].to_string();
        for j in scores {
            if a == j.0 && b == j.1 {
                n+= j.2;
            }
            if b == j.0 && a == j.1 {
                n+= j.2;
            }
        }
        i+=1;
    }
    if guest_arrangement.len() > 2 {
        let a = guest_arrangement[0].to_string();
        let b = guest_arrangement.last().unwrap().to_string();
        for i in scores {
            if a == i.0 && b == i.1 {
                n+= i.2;
            }
            if b == i.0 && a == i.1 {
                n+= i.2;
            }
        }
    }
    n
}
