fn main() {


    let mut s: String = String::from("1321131112");
    //part 1
    for _ in 0..40 {
        s = next_look_and_say(&s);
    }
    println!("{}", s.len());
    //part 2
    for _ in 0..10 {
        s = next_look_and_say(&s);
    }
    println!("{}", s.len());
}

fn next_look_and_say(sequence: &str) -> String {
    let mut new_seq = String::new();
    let mut n = 1;
    let mut i = 1;
    let chars = sequence.chars().collect::<Vec<char>>();
    let mut current_char : char = chars[0].clone(); // dummy value
    while i < chars.len() {
        if chars[i] == current_char {
            n+=1;
        } else {
            new_seq.push(std::char::from_digit(n, 10).unwrap());
            new_seq.push(current_char);
            current_char = chars[i].clone();
            n=1;
        }
        i+=1;
    }
    new_seq.push(std::char::from_digit(n, 10).unwrap());
    new_seq.push(current_char);
    new_seq
}
