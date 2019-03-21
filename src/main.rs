mod arbitrary;
mod core;

fn main () {
	let test = core::TestInstance::default_test();
	
	println!("{:?}", test);
}