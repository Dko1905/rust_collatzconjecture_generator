use std::fs::File;
use std::io::prelude::*;

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
	let max_number : usize;
	
	if args.len() != 3{
		println!("Usage : rcg \"result.csv\" 1000");
		return;
	}
	let file_name : String = args[1].clone();
	max_number = args[2].parse().expect("Error in parseing  to u128.");
	
	// Create buffer to store result in.
	let mut buffer : Vec<u128> = Vec::with_capacity(max_number);

	for i in 0..max_number{
		let mut len : u128 = 0;
		let mut number : u128 = i.clone() as u128;
		while number > 1{
			if check_if_even(number){
				if_even(&mut number);
			}
			else{
				if_odd(&mut number);
			}
			len += 1;
		}
		buffer.push(len);
	}

	// save buffer to file
	let mut file = File::create(file_name).expect("Error in creating file");
	
	for n in 0..buffer.len(){
		let element = buffer[n];
		file.write(format!("{},{}\n", n, element).as_bytes()).expect("Error in writing to file.");
	}
}
