use itertools::Itertools;

fn main() {
    let containers: Vec<i64> = vec![50,44,11,49,42,46,18,32,26,40,21,7,18,43,10,47,36,24,22,40];
    let liters = 150;
    let mut n = 0;
    for i in 0..containers.len() {
        for j in containers.iter().combinations(i) {
            if j.iter().fold(0, |acc, x| acc+(*x)) == liters {
                n+=1;
            }
        }
    }
    println!("part 1 : {}", n);

    // part 2
    let mut min_nbr : usize = 0;
    'a: for i in 0..containers.len() {
        for j in containers.iter().combinations(i) {
            if j.iter().fold(0, |acc, x| acc+(*x)) == liters {
                min_nbr = i;
                break 'a;
            }
        }
    }
    println!("{}", min_nbr);
    println!("{:?}", containers.iter()
                     .combinations(min_nbr)
                     .filter(|x| x.iter().fold(0, |acc, y| acc+(*y)) == liters)
                     .collect::<Vec<Vec<&i64>>>().len());
}
