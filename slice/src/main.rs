fn main() {
	// string slice 
	let s = String::from("broadcast");
	let part1 = &s[0..5];
	let part2 = &s[5..9];
	println!("{}={}+{}",s,part1,part2);
	/* 
	..y <==> 0..y
	x.. <==> x..end
	..  <==> 0..end
	 */
	// slice result must be reference

	// array slice
	let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}
