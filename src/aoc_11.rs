use fancy_regex::Regex;

fn main() {
    // part 1
    let mut s =  String::from("hepxcrrq");
    loop {
        s = inc_string(s.as_str());
        if is_valid(&s) {
            break;
        }
    }
    println!("{}", s);

    // part 2
    let mut s =  String::from("hepxxyzz");
    loop {
        s = inc_string(s.as_str());
        if is_valid(&s) {
            break;
        }
    }
    println!("{}", s);

}

fn is_valid(s: &str) -> bool {
    has_three_repeating_inc(s) &&
    hasnt_forbidden_letters(s) &&
    has_two_non_overlapping_pairs(s)
}

fn inc_string(s : &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len();
    let mut ret = 1;
    while ret > 0 && i > 0 {
        i-=1;
        if chars[i] == 'z' {
            chars[i] =  'a';
            ret+=1;
        } else {
            chars[i] = std::char::from_u32(chars[i] as u32 + 1).unwrap();
        }
        ret-=1;

    }
    chars.iter().collect()
}

fn has_three_repeating_inc(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }
    let chars: Vec<i64> = s.chars().map(|x| { x as i64 }).collect();
    let mut current_char: i64 = chars[0].clone();
    let mut n = 0;
    for i in chars.iter() {
        if (i - current_char) == 1 {
            n+=1;
        } else if (i - current_char) != 1 {
            n=0;
        }
        if n == 2 {
            return true;
        }
        current_char = i.clone();
    }
    false
}

fn hasnt_forbidden_letters(s: &str) -> bool {
    let forbidden = ['i','o', 'l'];
    for i in s.chars() {
        if forbidden.contains(&i) {
            return false;
        }
    }
    true
}

fn has_two_non_overlapping_pairs(s: &str) -> bool {
    let r = Regex::new(r#"(.)\1.*(.)\2"#).unwrap();
    r.is_match(s).unwrap()
}
