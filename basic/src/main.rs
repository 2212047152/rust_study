fn main() {
	// immutable variable declration but belongs to variable
	let a = 123;
	// mutable variable
	let mut b = 32;
	println!("Pre => a : {0}, b : {1}",a,b);
	
	b = 456;
	println!("Aft b => a : {0}, b : {1}",a,b);

	// shadowing immutable variable
	let a: i64 = 456;
	println!("Aft a => a : {0}, b : {1}",a,b);
	
	// constant declration
	const C: i32 = 123;
	println!("Pre a => a : {0}, b : {1}, c : {2}",a,b,C);


}
