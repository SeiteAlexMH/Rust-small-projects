use std::io;

fn main() {
	let mut even: u64 =0;
	let mut odd: u64 = 1;
	println!("What fibonacci number would you like?");
	let mut number = String::new();
	io::stdin().read_line(&mut number)
		.expect("Failed to read line");
	let number: u32 = match number.trim().parse(){
		Ok(num) => num,
		Err(_)  => 0,
	};
	if number == 0{
		println!("That Fibonacci number is: 0");
	} else {
		for counter in 1..number+1{
			if counter % 2 == 0{
				even += odd;
			} else {
				odd += even;
			}
		}
		if number % 2 == 0{
			println!("That Fibonacci number is: {}", even);
		} else {
			println!("That Fibonacci number is: {}", odd);
		}	
	}
}
