fn main() {
    println!("Hello, world!");
	another_function(12,16);

	let x = 5;
	let y = {
		x + 1
	};
	println!("x value => {x}, y value => {y}");

	// function defination can be nested
	fn five() -> i32 {
		5
	}
	println!("print fn five => {}",five());

	println!("another_function return => {}",another_function(5, 6));
}


fn another_function(x: i32,y: i32) -> i32{
	println!("x value => {x}, y value => {y}");
	return x + y;
}