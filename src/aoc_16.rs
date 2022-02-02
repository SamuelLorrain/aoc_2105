#[derive(Debug)]
struct Aunt {
    n: Option<i64>,
    children: Option<i64>,
    cats: Option<i64>,
    samoyeds: Option<i64>,
    pomeranians: Option<i64>,
    akitas: Option<i64>,
    vizslas: Option<i64>,
    goldfish: Option<i64>,
    trees: Option<i64>,
    cars: Option<i64>,
    perfumes: Option<i64>,
}

impl Aunt {
    fn parse(s : &str) -> Aunt {
        let v = s.split(&[',', ':'][..])
                 .map(|x| x.trim().to_string())
                 .collect::<Vec<String>>();
        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;
        for (i,w) in v.iter().enumerate() {
            match w.as_str() {
                "children" => children = Some(v[i+1].parse::<i64>().unwrap()),
                "cats" => cats = Some(v[i+1].parse::<i64>().unwrap()),
                "samoyeds" => samoyeds = Some(v[i+1].parse::<i64>().unwrap()),
                "pomeranians" => pomeranians = Some(v[i+1].parse::<i64>().unwrap()),
                "akitas" => akitas = Some(v[i+1].parse::<i64>().unwrap()),
                "vizslas" => vizslas = Some(v[i+1].parse::<i64>().unwrap()),
                "goldfish" => goldfish = Some(v[i+1].parse::<i64>().unwrap()),
                "trees" => trees = Some(v[i+1].parse::<i64>().unwrap()),
                "cars" => cars = Some(v[i+1].parse::<i64>().unwrap()),
                "perfumes" => perfumes = Some(v[i+1].parse::<i64>().unwrap()),
                _ => ()
            }
        }
        Aunt {
            n: Some(v[0].split(" ").last().unwrap().parse::<i64>().unwrap()),
            children: children.clone(),
            cats: cats.clone(),
            samoyeds: samoyeds.clone(),
            pomeranians: pomeranians.clone(),
            akitas: akitas.clone(),
            vizslas: vizslas.clone(),
            goldfish: goldfish.clone(),
            trees: trees.clone(),
            cars: cars.clone(),
            perfumes: perfumes.clone(),
        }
    }
}

fn main() {
    let to_find = Aunt {
        n: None,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    let mut v: Vec<Aunt> = vec![];
    let s = std::fs::read_to_string("./inputs/input_aoc_16.txt").unwrap();
    for i in s.lines() {
        v.push(Aunt::parse(i));
    }
     let part_1 = v.iter()
      .filter(|x| x.children == None || x.children == Some(3))
      .filter(|x| x.cats == None || x.cats == Some(7))
      .filter(|x| x.samoyeds == None || x.samoyeds == Some(2))
      .filter(|x| x.pomeranians == None || x.pomeranians == Some(3))
      .filter(|x| x.akitas == None || x.akitas == Some(0))
      .filter(|x| x.vizslas == None || x.vizslas == Some(0))
      .filter(|x| x.goldfish == None || x.goldfish == Some(5))
      .filter(|x| x.trees == None || x.trees == Some(3))
      .filter(|x| x.cars == None || x.cars == Some(2))
      .filter(|x| x.perfumes == None || x.perfumes == Some(1))
      .collect::<Vec<&Aunt>>();
     for i in part_1.iter() {
         println!("{:?}", i);
     }
     println!("{:?}", part_1.len());
    let part_2 = v.iter()
     .filter(|x| x.children == None || x.children == Some(3))
     .filter(|x| x.cats == None || x.cats > Some(7))
     .filter(|x| x.samoyeds == None || x.samoyeds == Some(2))
     .filter(|x| x.pomeranians == None || x.pomeranians < Some(3))
     .filter(|x| x.akitas == None || x.akitas == Some(0))
     .filter(|x| x.vizslas == None || x.vizslas == Some(0))
     .filter(|x| x.goldfish == None || x.goldfish < Some(5))
     .filter(|x| x.trees == None || x.trees > Some(3))
     .filter(|x| x.cars == None || x.cars == Some(2))
     .filter(|x| x.perfumes == None || x.perfumes == Some(1))
     .collect::<Vec<&Aunt>>();
    for i in part_2.iter() {
        println!("{:?}", i);
    }
    println!("{:?}", part_2.len());
}
