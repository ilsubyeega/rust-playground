fn main() {
	let _normal = 1;
	let _normal_2 = "asdsa";
	let _first_option : Option<isize> = None;
	let _another_option = Some(1);
    let _another_2_option = Some(6);
    let _another_3_option = Some("Hello World");
	dbg!(_first_option);
	dbg!(_another_2_option);
	println!("{:?}", _another_2_option);
}