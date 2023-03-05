
mod solve_pave;

use std::{thread::sleep, time::Duration, fmt};

use rand::Error;
use solve_pave::solve_pave::{solve_with_rand_sqr, solve_with_pre_sqr};

#[derive(Clone, PartialEq)]
enum ThreeArgAux {NoMistake,FormatMistake}

impl fmt::Display for ThreeArgAux {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ThreeArgAux::FormatMistake => write!(f, "^^^"),
			ThreeArgAux::NoMistake => write!(f,  "   ")
		}
    }
}


fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 && args.len() != 4  {
		println!("Incorrect Format");
		println!("cargo run [k] | cargo run [k] [x] [y]");
		return;
	}
	if args.len() == 2 {
		let k2  = args[1].parse();
		match k2 {
			Ok(k) => {
				println!("Solving with random special square position");
				solve_with_rand_sqr(k);
			},
			Err(_) => {
				println!("Invalid Digit");
				println!("Defulting to k = 3");
				println!("The exetucion will continue in 3 seconds");
				sleep(Duration::from_secs(3));
				solve_with_rand_sqr(3);
			}
		}
	} else {
		let mut k4 = vec![Ok(0);3];
		for i in 1..4 {
			k4[i-1] = args[i].parse::<u32>();
		}
		let mut mistake_cache : Vec<ThreeArgAux> = vec![ThreeArgAux::NoMistake;3];
		let mut values = vec![0;3];
		for i in 0..k4.len() {
			match k4[i] {
				Ok(m) => values[i] = m,
    			Err(_) => mistake_cache[i] = ThreeArgAux::FormatMistake, 
			}
		}
		if mistake_cache.contains(&ThreeArgAux::FormatMistake) {
				println!("Format Mistakes in cargo [k] [x] [y]");
				println!("                         {} {} {}", mistake_cache[0], mistake_cache[1], mistake_cache[2]);
				println!("Terminating execution");
				return;
		}
		solve_with_pre_sqr(values[0], values[1] as i32, values[2] as i32);
	}
}
