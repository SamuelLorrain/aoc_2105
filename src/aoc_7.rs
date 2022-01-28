use std::collections::HashMap;
use std::fs;

fn main() {
    let mut map: HashMap<String, u16> = HashMap::new();
    let mut content = fs::read_to_string("inputs/input_aoc_7.txt").unwrap();
    exec(content.as_str(), &mut map);
    // part 1
    println!("{}", map["a"]);

    // part 2
    let a = map["a"];
    map = HashMap::new();
    content.push_str(format!("{} -> b", a).as_str());
    exec(content.as_str(), &mut map);
    println!("{}", map["a"]);
}

fn exec(script : &str, map: &mut HashMap<String, u16>) {
    let mut lines: Vec<&str> = script.lines().collect();
    let mut i = 0;
    loop {
        while i < lines.len() {
            match process(lines[i], map)  {
                true => {
                    lines.remove(i);
                    ()
                },
                false => i+=1
            }
        }
        if lines.len() > 0 {
            i = 0;
        } else {
            break;
        }
    }
}

#[derive(Debug)]
enum Command {
    Assign,
    And,
    Not,
    Or,
    RShift,
    LShift
}

fn process(s : &str, context: &mut HashMap<String, u16>) -> bool {
    let command: Command;
    if s.matches("AND").collect::<Vec<&str>>().len() > 0 {
        command = Command::And;
    } else if s.matches("OR").collect::<Vec<&str>>().len() > 0 {
        command = Command::Or;
    } else if s.matches("RSHIFT").collect::<Vec<&str>>().len() > 0 {
        command = Command::RShift;
    } else if s.matches("LSHIFT").collect::<Vec<&str>>().len() > 0 {
        command = Command::LShift;
    } else if s.matches("NOT").collect::<Vec<&str>>().len() > 0 {
        command = Command::Not;
    } else { // assign
        command = Command::Assign;
    }
    let tokens: Vec<&str> = s.trim().split(' ').collect();
    match command {
        Command::Assign => {
            match from_var(tokens[0],  context) {
                Some(x) => {
                    context.insert(tokens[2].to_string(), x);
                    return true;
                },
                None => return false,
            }
        },
        Command::Not => {
            match from_var(tokens[1],  context) {
                Some(x) => {
                    context.insert(tokens[3].to_string(), !x);
                    return true;
                },
                None => return false,
            }
        },
        Command::LShift => {
            let sa = from_var(tokens[0],  context);
            let sb = from_var(tokens[2],  context);
            let a : u16;
            let b : u16;
            match sa {
                Some(x) => a = x,
                None => return false,
            }
            match sb {
                Some(x) => b = x,
                None => return false,
            }
            context.insert(tokens[4].to_string(), a << b);
            return true;
        },
        Command::RShift => {
            let sa = from_var(tokens[0],  context);
            let sb = from_var(tokens[2],  context);
            let a : u16;
            let b : u16;
            match sa {
                Some(x) => a = x,
                None => return false,
            }
            match sb {
                Some(x) => b = x,
                None => return false,
            }
            context.insert(tokens[4].to_string(), a >> b);
            return true;
        },
        Command::And => {
            let sa = from_var(tokens[0],  context);
            let sb = from_var(tokens[2],  context);
            let a : u16;
            let b : u16;
            match sa {
                Some(x) => a = x,
                None => return false,
            }
            match sb {
                Some(x) => b = x,
                None => return false,
            }
            context.insert(tokens[4].to_string(), a & b);
            return true;
        },
        Command::Or => {
            let sa = from_var(tokens[0],  context);
            let sb = from_var(tokens[2],  context);
            let a : u16;
            let b : u16;
            match sa {
                Some(x) => a = x,
                None => return false,
            }
            match sb {
                Some(x) => b = x,
                None => return false,
            }
            context.insert(tokens[4].to_string(), a | b);
            return true;
        },
    }
    panic!("process {:?} - {:?}", s, command);
    false
}

fn from_var(var_name : &str, context: &mut HashMap<String, u16>) -> Option<u16> {
    if let Ok(x) = var_name.parse::<u16>() {
        return Some(x);
    }
    if let Some(x) = context.get(var_name) {
        return Some(*x);
    }
    None
}
