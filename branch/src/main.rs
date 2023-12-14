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
	let a: [i32; 5] = [10,20,30,40,50];
	for i in a.iter() {
		println!("a => {i}")
	}

	for i in 0..5 {
		println!("a[{i}] = {}",a[i]);
	}	

	// infinate loop
    let s: [char; 6] = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i: usize = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }

	let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" The index of \'O\' is {}", location);
}
