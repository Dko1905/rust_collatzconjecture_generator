use std::fs::File;
use std::io::prelude::*;

use indicatif::ProgressBar;

#[inline]
fn check_if_even(n : u128) -> bool{
	n & 1 == 0
}

#[inline]
fn if_even(n : &mut u128){
	*n >>= 1;
}
#[inline]
fn if_odd(n : &mut u128){
	*n = *n * 3 + 1;
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let max_number : u128;
	
	if args.len() != 3{
		println!("Usage : rcg \"result.csv\" 1000");
		return;
	}
	let file_name : String = args[1].clone();
	max_number = args[2].parse().expect("Error in parseing  to u128.");
	let mut file = File::create(file_name).expect("Error in creating file");

	let pb = ProgressBar::new((max_number/100) as u64);

	for i in 0..max_number{
		let mut len = 0;
		let mut number = i.clone();
		while number > 1{
			if check_if_even(number){
				if_even(&mut number);
			}
			else{
				if_odd(&mut number);
			}
			len += 1;
		}
		file.write_all(format!("{},{}", i, len).as_bytes()).expect("Could not write to file");
		if i % 100 == 0{
			pb.inc(1);
		}
	}
	pb.finish_with_message("done");
}
