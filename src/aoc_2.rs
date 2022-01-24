use std::fs;

fn main() {
    // println!("{}", compute_wrapping_size(2,3,4));
    // println!("{}", compute_wrapping_size(1,1,10));
    // println!("{:?}", get_dimensions("20x3x11"));

    let content = fs::read_to_string("../input_aoc_2.txt").unwrap();

    let mut sum : u64 = 0;
    let mut sum_ribbon : u64 = 0;
    for line in content.lines() {
        let (x,y,z) = get_dimensions(line);
        sum += compute_wrapping_size(x,y,z);
        sum_ribbon += compute_ribbon_size(x,y,z);
    }
    println!("{}", sum);
    println!("{}", sum_ribbon);
}

fn min(mut v: Vec<u64>) -> u64 {
    v.sort();
    v.first().unwrap().clone()
}

fn compute_wrapping_size(l: u64, w: u64, h : u64) -> u64 {
    let x = 2*l*w;
    let y = 2*w*h;
    let z = 2*h*l;

    x + y + z + min(vec![l*w,w*h,h*l])
}

fn get_dimensions(s: &str) -> (u64,u64,u64) {
    let v: Vec<u64> = s
        .split('x')
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    (v[0].clone(),v[1].clone(),v[2].clone())
}

fn compute_ribbon_size(l: u64, w: u64, h: u64) -> u64 {
    let mut v = vec![l,w,h];
    v.sort();
    v[0].clone()*2+v[1].clone()*2+(l*w*h)
}
