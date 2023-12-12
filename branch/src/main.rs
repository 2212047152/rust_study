fn main() {
	// if condition
	let number = 3;
	if number < 5 {
		println!("IF true");
	} else {
		println!("IF false");
	}

	// if bool condition
	let condition = true;
	if condition {
		println!("Condition true");
	} else {
		println!("Condition false");
	}

	// if tristate expression
	let result = if condition {true} else {false};
	println!("Result is {}",result);

	// while condition
	let mut flag = 1;
	while flag != 4 {
		flag += 1;
		println!("flag is {flag}")
	}

	// for condition
	
}
