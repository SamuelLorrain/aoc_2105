use std::fs;

fn main() {
    let size = 1000;
    let mut n = 0;
    let mut map: Vec<Vec<i64>> = vec![vec![0; size]; size];
    let content = fs::read_to_string("inputs/input_aoc_6.txt").unwrap();

    for i in content.lines() {
        let c = Command::parse(i);
        c.exec(&mut map);
    }
    for i in map {
        for j in i {
            n += j;
        }
    }
    println!("{}", n);
}

#[derive(Debug)]
enum CommandType {
    Toggle,
    TurnOn,
    TurnOff
}

#[derive(Debug)]
struct Command {
    range_x: Vec<usize>,
    range_y: Vec<usize>,
    command: CommandType
}

impl Command {
    fn exec(&self, g: &mut Vec<Vec<i64>>) {
        for i in self.range_y[0]..=self.range_y[1] {
            for j in self.range_x[0]..=self.range_x[1] {
                match &self.command {
                    CommandType::Toggle => g[i][j] += 2,
                    CommandType::TurnOn => g[i][j] += 1,
                    CommandType::TurnOff => {
                        if g[i][j] > 0 {
                            g[i][j] -= 1;
                        }
                    }
                }
            }
        }
    }

    fn parse_range(s: &str) -> Vec<usize> {
        let x = s.split(',')
                 .map(|x| { x.parse::<usize>().unwrap() })
                 .collect::<Vec<usize>>();
        if x.len() != 2 {
            panic!("Invalid Token");
        }
        x
    }

    fn parse(s: &str) -> Command {
        let splitted = s.split(' ').collect::<Vec<&str>>();
        let command: CommandType;
        let first_vec: Vec<usize>;
        let second_vec: Vec<usize>;
        // Compute commandType
        if splitted.len() == 5 {
            if splitted[1] == "on" {
                command = CommandType::TurnOn;
            } else if splitted[1] == "off" {
                command = CommandType::TurnOff;
            } else {
                panic!("Invalid token");
            }

            first_vec = Command::parse_range(splitted[2]);
            second_vec = Command::parse_range(splitted[4]);
        } else if splitted.len() == 4 {
            if splitted[0] == "toggle" {
                command = CommandType::Toggle;
            } else {
                panic!("Invalid token");
            }
            first_vec = Command::parse_range(splitted[1]);
            second_vec = Command::parse_range(splitted[3]);
        } else {
            panic!("Invalid token");
        }
        let range_x: Vec<usize> = vec![first_vec[0], second_vec[0]];
        let range_y: Vec<usize> = vec![first_vec[1], second_vec[1]];

        Command {
            range_x,
            range_y,
            command
        }
    }
}

