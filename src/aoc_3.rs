use std::fs;

fn main() {
    let mut g = Grid::new(1000);

    let content = fs::read_to_string("../input_aoc_3.txt").unwrap();
    // let content = "^v^v^v^v^v";

    //compute_houses(&content, &mut g);
    compute_houses_pair(&content, &mut g, true);
    g.reset_pos();
    compute_houses_pair(&content, &mut g, false);
    for i in g.map.iter() {
        println!("{:?}", i);
    }

    println!("{}", count_houses_delivered(&g));

    // println!("{}", count_houses_delivered(&g));
}

fn compute_houses(s : &str, g : &mut Grid) {
    g.inc();
    for (i,c) in s.chars().enumerate() {
        match c {
            '^' => g.up(),
            'v' => g.down(),
            '<' => g.left(),
            '>' => g.right(),
            _ => break
        }
        g.inc();
    }
}

fn compute_houses_pair(s : &str, g : &mut Grid, pair : bool) {
    g.inc();
    for (i,c) in s.chars().enumerate() {
        if (pair && i % 2 == 0) || (!pair && i % 2 != 0) {
            match c {
                '^' => g.up(),
                'v' => g.down(),
                '<' => g.left(),
                '>' => g.right(),
                _ => break
            }
            g.inc();
        }
    }
}

fn count_houses_delivered(g: &Grid) -> usize{
    let mut n: usize = 0;
    for i in g.map.iter() {
        for j in i.iter() {
            if *j > 0 {
                n += 1;
            }
        }
    }
    n
}

struct Grid {
    position_x: usize,
    position_y: usize,
    map: Vec<Vec<usize>>,
    size: usize
}
impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            position_x : size/2,
            position_y : size/2,
            map: vec![vec![0 ; size]; size],
            size
        }
    }

    fn reset_pos(&mut self){
        self.position_y = self.size/2;
        self.position_x = self.size/2;
    }

    fn up(&mut self) { self.position_y -= 1; }
    fn down(&mut self) { self.position_y += 1; }
    fn left(&mut self) { self.position_x -= 1; }
    fn right(&mut self) { self.position_x += 1; }
    fn inc(&mut self) {
        self.map[self.position_y][self.position_x] += 1;
    }
}

