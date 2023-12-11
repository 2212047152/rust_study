
/* 
	Integer:
	Arch		isize	usize
	8-bit	:	i8		u8 
	16-bit	:	i16		u16
	32-bit	:	i32		u32
	64-bit	:	i64		u64
	128-bit	:	i128	u128
*/

/* 
	Integer represent methods:
	Decimal		:	98_222 	(able to insert '_' in numer)
	Hexadecimal	:	0xff
	Octal		:	0o77
	Binary		:	0b1101_0011
	Byte		:	b'A'	(only for u8)
*/

/* 
	float represent methods : f32 and f64 (default type for x.0 num repersentation) 
*/

/* 
	bool represent methods : bool (only valid for true and false)
*/

/* 
	tuple represent methods : let tup: (i32, f64, u8) = (500, 6.4, 1);
	tuple can conclude different data types
*/

/* 
	array represent methods : let a: [i32; 5] = [1, 2, 3, 4, 5]; (i32 type with 5 mumber)
	tuple can conclude same data types
*/


/* 
	rust support : a += 1;
	rust does not support : a++
*/

fn main() {
    let a:u8 = 255;
	println!("a for u8 : {}", a);
	
	let b = 1.0;	// f64
	println!("b for fp64 : {}", b);
	let b: f32 = 1.0;	// f32
	println!("b for fp32 : {}", b);

	let c = 'c';
	println!("c for char (4B): {}",c);

	let tup = ("hello", 6.4, 1);
	let (x,y,z) = tup;
	println!("tuple : x=>{0}, y=>{1}, z=>{2}",tup.0,tup.1,tup.2);
	println!("deconstruct : x=>{0}, y=>{1}, z=>{2}",x,y,z);

	let array: [i32; 5] = [1, 2, 3, 4, 5];
	println!("Pre => array : array[0] : {0}, array[1] : {1}", array[0], array[1]);
	let mut array: [i32; 5] = array;
	array[0] = 6;
	println!("Aft => array : array[0] : {0}, array[1] : {1}", array[0], array[1]);

}
