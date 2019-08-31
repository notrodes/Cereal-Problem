use std::{io::stdin, time::{Duration, SystemTime}};

use rand::{Rng, thread_rng};

fn simulation(number_of_loops: usize) -> (usize, usize, f64, Duration) {
    let mut probability = Vec::new();
    // Start timer
    let start = SystemTime::now();
    // Simulation loop
    for _i in 0..number_of_loops {
        let mut prizes: [usize; 6] = [0, 0, 0, 0, 0, 0];
        let mut opens: usize = 0;
        // loop until owns every prize
        while prizes.contains(&0) {
            prizes[thread_rng().gen_range(0, 6)] += 1;
            opens += 1;
        }
        probability.push(opens);
    }
    // End timer
    let timer = start.elapsed().unwrap();
    (*probability.iter().max().unwrap(), *probability.iter().min().unwrap(), (probability.iter().sum::<usize>()) as f64 / probability.len() as f64, timer)
}

fn main() {
    loop {
        // User input and parsing
        println!("How many times to loop? or press [q] to quit.");
        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Could not read input.");
        if user_input.trim() == "q" {
            break;
        }
        let user_input = user_input.trim().parse::<usize>();
        match user_input {
            // Run main code if parse is Ok()
            Ok(user_input) => {
                let stats = simulation(user_input);
                println!("Max; {} Min; {} \nThe average {}\nTime elapsed; {:?}", stats.0, stats.1, stats.2, stats.3)
            }
            // If input is not usize
            Err(_error) => {
                eprint!("Input must be integer.\n");
            }
        }
    }
}
