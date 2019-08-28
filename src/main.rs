use std::{io, time};
use rand::{Rng, thread_rng};

fn main() {
    println!("How many times to loop?");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read input.");
    let user_input = user_input.trim().parse::<u128>().unwrap();
    let mut probability = Vec::new();
    let start = time::SystemTime::now();
    for _i in 0..user_input {
        let mut prizes = [0, 0, 0, 0, 0, 0];
        let mut opens:usize = 0;
        while prizes.contains(&0) {
            prizes[thread_rng().gen_range(0, 6)] += 1;
            opens += 1;
        }
        probability.push(opens);
    }
    let timer = start.elapsed().unwrap();
    println!("Max; {} Min; {} \nThe average {}\nTime elapsed; {:?}",
             probability.iter().max().unwrap(), probability.iter().min().unwrap(),
             (probability.iter().sum::<usize>()) as f64 / probability.len() as f64,
             timer);
    let mut exit = String::new();
    io::stdin()
        .read_line(&mut exit)
        .expect("Could not read input.");
}
