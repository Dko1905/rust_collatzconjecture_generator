use std::sync::mpsc;
use std::thread;
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
	let threads : u8;
	let file_writer_handle : std::thread::JoinHandle<()>;
	let max_number : u128;
	let args: Vec<String> = std::env::args().collect();
	
	if args.len() != 3{
		println!("usage: rust_collatzconjecture_generator max_number threads");
		return;
	}
	max_number = args[1].parse().expect("Error in parsing input");
	threads = args[2].parse().expect("Error in parsing input");
	println!("Running on {} threads, max is {}.", threads, max_number);

	{
		let (tx, rx) = mpsc::channel::<(u128,u128)>();	
		let mut handles : Vec<std::thread::JoinHandle<()>> = Vec::new();

		file_writer_handle = thread::spawn(move || {
			let mut file = File::create("result.csv").unwrap();
			for message in rx.iter(){
				file.write_all(format!("{},{}\n", message.0, message.1).as_bytes()).unwrap();
			}
		});

		for n in 0..threads{
			let tx_clone = tx.clone();
			let thread_number = n.clone();
			let threads = threads.clone();
			let max_number = max_number.clone();

			let handle = thread::spawn(move || {
				for number in ((thread_number as u128)..max_number).step_by(threads as usize){
					let mut n = number.clone();
					let mut len = 1;
					while n > 1{
						if check_if_even(n){
							if_even(&mut n);
						}
						else{
							if_odd(&mut n);
						}
						len += 1;
					}
					tx_clone.send((number, len)).unwrap();
				}
				
			});

			handles.push(handle);
		}
		
		for handle in handles{
			handle.join().unwrap();
		}
	}
	
	file_writer_handle.join().unwrap();

}
