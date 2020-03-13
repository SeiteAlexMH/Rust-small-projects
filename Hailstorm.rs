use std::io;

fn main() {
	println!("What hailstorm number would you like?");
	let mut number = String::new();
	io::stdin().read_line(&mut number)
		.expect("Failed to read line");
	let mut number: u64 = match number.trim().parse(){
		Ok(num) => num,
		Err(_)  => 0,
	};
	let mut counter = 0;
	loop {
		println!("{}: {}",counter,number);
		if number == 1{
			break;
		};
		counter += 1;
		if number % 2 == 0{
			number/=2;
		} else {
			number = number*3+1;
		};
	};
	println!("It took {} steps to go down to go down to one", counter);
}
