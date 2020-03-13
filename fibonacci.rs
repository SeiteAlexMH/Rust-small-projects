use std::io;
fn main() {
	println!("What fibonacci number would you like?");
	let mut number = String::new();
	io::stdin().read_line(&mut number)
		.expect("Failed to read line");
	let number: u64 = match number.trim().parse(){
		Ok(num) => num,
		Err(_)  => 0,
	};
	println!("That Fibonacci number is: {}", fib(number));
}

fn fib(x:u64) -> u64{
	if x < 3{
		1
	} else {
		fib(x-1) + fib(x-2)
	}
}
