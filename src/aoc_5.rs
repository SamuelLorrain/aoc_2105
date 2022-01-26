use std::fs;
use fancy_regex::Regex;

fn main() {
    let content = fs::read_to_string("src/inputs/input_aoc_5.txt").unwrap();
    let mut n = 0;
    for i in content.lines() {
        if is_nice_two(i) {
            n+=1;
        }
    }
    println!("{}", n);
}

fn is_nice(s: &str) -> bool {
    contains_three_vowels(s) &&
    a_letter_appears_twice(s) &&
    doesnt_contains_bad_strings(s)
}

fn is_nice_two(s: &str) -> bool {
    two_non_overlapping_pairs(s) &&
    letter_appear_twice_two_by_two(s)
}

fn contains_three_vowels(s: &str) -> bool {
    let r = Regex::new(r#"a|e|i|o|u"#).unwrap();
    let mut n = 0;
    for _i in r.find_iter(s) {
        n += 1;
    }
    n >= 3
}

fn a_letter_appears_twice(s :&str) -> bool {
    let r = Regex::new(r#"(.)\1"#).unwrap();
    r.is_match(s).unwrap()
}

fn doesnt_contains_bad_strings(s : &str) -> bool {
    let r = Regex::new(r#"ab|cd|pq|xy"#).unwrap();
    !r.is_match(s).unwrap()
}

fn two_non_overlapping_pairs(s : &str) -> bool {
    let r = Regex::new(r#"(..).*\1"#).unwrap();
    r.is_match(s).unwrap()
}

fn letter_appear_twice_two_by_two(s: &str) -> bool {
    let r = Regex::new(r#"(.).\1"#).unwrap();
    r.is_match(s).unwrap()
}

