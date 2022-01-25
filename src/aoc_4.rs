use md5;

fn main() {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("ckczppom{}", i).as_bytes());
        let repr = format!("{:2x}", digest);
        if repr.starts_with("000000") {
            break;
        }
        i+=1;
    }
    println!("{}", i);


    let digest = md5::compute("ckczppom117946");
    println!("{:2x}", digest);
}
