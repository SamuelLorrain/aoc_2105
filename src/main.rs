use std::collections::HashMap;
use std::collections::HashSet;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;

fn main() {
    let input_s = "ORnPBPMgArCaCaCaSiThCaCaSiThCaCaPBSiRnFArRnFArCaCaSiThCaCaSiThCaCaCaCaCaCaSiRnFYFArSiRnMgArCaSiRnPTiTiBFYPBFArSiRnCaSiRnTiRnFArSiAlArPTiBPTiRnCaSiAlArCaPTiTiBPMgYFArPTiRnFArSiRnCaCaFArRnCaFArCaSiRnSiRnMgArFYCaSiRnMgArCaCaSiThPRnFArPBCaSiRnMgArCaCaSiThCaSiRnTiMgArFArSiThSiThCaCaSiRnMgArCaCaSiRnFArTiBPTiRnCaSiAlArCaPTiRnFArPBPBCaCaSiThCaPBSiThPRnFArSiThCaSiThCaSiThCaPTiBSiRnFYFArCaCaPRnFArPBCaCaPBSiRnTiRnFArCaPRnFArSiRnCaCaCaSiThCaRnCaFArYCaSiRnFArBCaCaCaSiThFArPBFArCaSiRnFArRnCaCaCaFArSiRnFArTiRnPMgArF";
    let rules_s = r#"Al => ThF
Al => ThRnFAr
B => BCa
B => TiB
B => TiRnFAr
Ca => CaCa
Ca => PB
Ca => PRnFAr
Ca => SiRnFYFAr
Ca => SiRnMgAr
Ca => SiTh
F => CaF
F => PMg
F => SiAl
H => CRnAlAr
H => CRnFYFYFAr
H => CRnFYMgAr
H => CRnMgYFAr
H => HCa
H => NRnFYFAr
H => NRnMgAr
H => NTh
H => OB
H => ORnFAr
Mg => BF
Mg => TiMg
N => CRnFAr
N => HSi
O => CRnFYFAr
O => CRnMgAr
O => HP
O => NRnFAr
O => OTi
P => CaP
P => PTi
P => SiRnFAr
Si => CaSi
Th => ThCa
Ti => BP
Ti => TiTi
e => HF
e => NAl
e => OMg"#;
    let mut rules : HashMap<String, Vec<String>> = HashMap::new();
    for i in rules_s.lines() {
        let v = i.split(" => ").collect::<Vec<&str>>();
        match rules.get_mut(&v[0].to_string()) {
            Some(x) => {
                x.push(v[1].to_string());
            },
            None => {
                rules.insert(v[0].to_string(), vec![v[1].to_string()]);
            }
        }
    }

    // part 1
    // unsafe {
    //     let mut set : HashSet<String> = HashSet::new();
    //     for (k,v) in rules.iter() {
    //         for rule in v.iter() {
    //             for m in input_s.match_indices(k) {
    //                 let s = input_s.get_unchecked(..m.0);
    //                 let s_two = input_s.get_unchecked(m.0+m.1.len()..);
    //                 set.insert(format!("{}{}{}", s, rule, s_two));
    //             }
    //         }
    //     }
    // }

    //part2
    let mut s: String = input_s.to_string();
    let mut n = 0;
    let mut exploded_rules = rules_s.lines().map(|x| x.split(" => ").collect::<Vec<&str>>()).collect::<Vec<_>>();
    // biggest to lowest length rule
    exploded_rules.sort_by(|a,b| {
        if (a[0].len() + a[1].len()) > (b[0].len() + b[1].len()) {
            return Ordering::Less;
        } else if (a[0].len() + a[1].len()) < (b[0].len() + b[1].len()) {
            return Ordering::Greater;
        }
        Ordering::Equal
    });
    // println!("{}", exploded_rules)
    while s != "e" {
        for i in  exploded_rules.iter() {
            let key = i[0];
            let value = i[1];

            while s.contains(value) {
                println!("{}", value);
                s = s.replacen(value, key, 1);
                n+=1;
            }
        }
        println!("s.len() => {}", s.len());
        println!("s : {}", s);
        println!("{}", n);
        exploded_rules.shuffle(&mut thread_rng());
    }
    println!("{}", n);
}


