fn main() {
    // data move in stack
	// both data will be reserved
	let x = 5;
	let y = x;
	println!("Stack : x is {x}, y is {y}");

	// data move in heap
	// after moved previous variable is stale
	let s1 = String::from("Hello");
	let s2 = s1;
	// s1 is staled
	// println!("{s1}, world");
	println!("Move : {s2}, world");

	// data clone
	// this operation can avoid stale
	let s1 = String::from("Hello");
	let s2 = s1.clone();
	println!("Clone : s1 = {s1}, s2 = {s2}");

	// some complicated situation
	let s = String::from("Hello");
	takes_ownership(s);
	// s value is tranfer into function
	// s can be seen as moved and invalid here

	let x = 5;
	let y = makes_copy(x);
	// x is basic type, it will valid here
	// y get the ownership of the return value in function calling
	println!("makes_copy return : y => {y}");


	// Reference which cannot change value in reference
	let s1 = String::from("hello");
	let s2 = &s1;
	println!("Reference : s2 => {s2}");

	let s = String::from("hello");
	let len = calculate_length(&s);
	println!("The length of '{}' is {}.", s, len);

	let s1 = String::from("hello");
	let mut s2 = &s1;
	println!("Preborrow : s2 => {}", s2);
	let s3 = s1;
	s2 = &s3; // reborrow ownership from s3 because s1 is staled
	println!("Reborrow : s2 => {}", s2);
	let mut s1 = String::from("Hello");
    // s1 is mutable
    let s2 = &mut s1;
    // s2 is mutable reference
    s2.push_str(" world");
    println!("mut reference : s2 -> {}", s2);

	/* 
		mut reference is banned for mutiple reference : avoid for data collision
		immute reference is allowed for multiple reference
	 */

	/* 
		rust is banned for dangle reference -> refer to a void varibale or variables which is free
	 */
	 
}

fn takes_ownership(some_string: String){
	println!("takes_ownership : s => {some_string}");
	// s will be released here
}

fn makes_copy(some_integer: i32) -> i32{
	// get a copy of x
	println!("makes_copy : x => {some_integer}");
	some_integer
}

fn calculate_length(s: &String) -> usize {
    s.len()
}