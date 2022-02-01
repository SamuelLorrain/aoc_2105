use fancy_regex::Regex;
use std::fs;
use serde_json::{Value};

fn main() {
    let s = fs::read_to_string("./src/inputs/input_aoc_12.txt").unwrap();
    part_1(&s);
    part_2(&s);
}

fn part_1(s: &str) {
    let r = Regex::new(r#"-?\d+"#).unwrap();
    let mut n: i64 = 0;
    for i in r.find_iter(s) {
        n+= i.unwrap().as_str().parse::<i64>().unwrap();
    }
    println!("{}", n);
}

fn part_2(s: &str) {
    let mut js: Value = serde_json::from_str(s).unwrap();
    let n;
    n = traverse_sum(&mut js);
    println!("{}", n);
}

fn traverse_sum(js: &mut Value) -> i64 {
    let mut n: i64 = 0;
    if js.is_array() {
        for v in js.as_array_mut().unwrap() {
            match v {
                Value::Null => (),
                Value::Bool(_) => (),
                Value::String(_) => (),
                Value::Number(x) => {
                    n += x.as_i64().unwrap();
                },
                Value::Array(_) => {
                    n += traverse_sum(&mut *v);
                },
                Value::Object(_) => {
                    n += traverse_sum(&mut *v);
                }
            }
        }
        return n;
    }
    let mut has_red = false;
    for (_,v) in js.as_object().unwrap() {
        if v.is_string() && v.as_str().unwrap() ==  "red" {
            has_red = true;
        }
    }
    if has_red {
        js.as_object_mut().unwrap().clear();
    }
    for (_,v) in js.as_object_mut().unwrap() {
        match v {
            Value::Null => (),
            Value::Bool(_) => (),
            Value::Number(x) => {
                n += x.as_i64().unwrap();
            },
            Value::String(s) => {
                if s == "red" && v.is_object() {
                    return 0;
                }
            },
            Value::Array(_) => {
                n += traverse_sum(&mut *v);
            },
            Value::Object(_) => {
                n += traverse_sum(&mut *v);
            }
        }
    }
    n
}
