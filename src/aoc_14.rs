#[derive(Debug, Clone)]
struct Reindeer {
    speed: i64,
    stamina: i64,
    resting: i64
}

fn main() {
    let s = std::fs::read_to_string("./inputs/input_aoc_14.txt").unwrap();
    // let s = "Comet can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    //part 1
    let mut max = std::i64::MIN;
    for i in s.lines() {
        if max < calc_dist(&parse(i), 2503) {
            max = calc_dist(&parse(i), 2503);
        }
    }
    println!("{}", max);
    let deers : Vec<Reindeer> = s.lines().map(parse).collect();
    part_2(&deers, 2503);
}

fn part_2(v: &Vec<Reindeer>, time: i64) {
    let mut deers : Vec<(Reindeer, Reindeer, i64, i64)> = vec![];
    let mut elapsed = 0;
    for i in v {
        deers.push((i.clone(),i.clone(), 0, 0));
    }
    while elapsed < time {
        for (deer,new_deer,km,_) in deers.iter_mut() {
            if new_deer.stamina > 0 {
                *km+=new_deer.speed;
                new_deer.stamina-=1;
            } else if new_deer.stamina <= 0 && new_deer.resting > 0 {
                new_deer.resting-=1;
            }
            if new_deer.stamina <= 0 && new_deer.resting <= 0 {
                new_deer.stamina = deer.stamina;
                new_deer.resting = deer.resting;
            }

        }
        // compute score
        let mut max = std::i64::MIN;
        for (_,_,km,_) in deers.iter() {
            if max < *km {
                max = *km;
            }
        }
        for (_,_,km,score) in deers.iter_mut() {
            if *km == max {
                *score+=1;
            }
        }
        elapsed+=1;
    }
    let mut max = std::i64::MIN;
    for (_,_,_,score) in deers.iter() {
        if max < *score {
            max = *score;
        }
    }
    println!("{:?}", max);
}

fn calc_dist(r: &Reindeer, time: i64) -> i64 {
    let mut elapsed = 0;
    let mut n = 0;
    let mut new_r = r.clone();
    while elapsed < time {
        if new_r.stamina > 0 {
            n+=new_r.speed;
            new_r.stamina-=1;
        } else if new_r.stamina <= 0 && new_r.resting > 0 {
            new_r.resting-=1;
        }
        if new_r.stamina <= 0 && new_r.resting <= 0 {
            new_r.stamina = r.stamina;
            new_r.resting = r.resting;
        }
        elapsed +=1;
    }
    n
}

fn parse(s: &str) -> Reindeer {
    let v: Vec<&str> = s.split(' ').collect();
    Reindeer {
        speed: v[3].parse::<i64>().unwrap(),
        stamina: v[6].parse::<i64>().unwrap(),
        resting: v[13].parse::<i64>().unwrap(),
    }
}
