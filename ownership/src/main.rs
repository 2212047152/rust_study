fn main() {
    // data move in stack
	// both data will be reserved
	let x = 5;
	let y = x;
	println!("x is {x}, y is {y}");

	// data move in heap
	// after moved previous variable is stale
	let s1 = String::from("Hello");
	let s2 = s1;
	// s1 is staled
	// println!("{s1}, world");
	println!("{s2}, world");

	// data clone
	// this operation can avoid stale
	let s1 = String::from("Hello");
	let s2 = s1.clone();
	println!("s1 = {s1}, s2 = {s2}");


}
