mod arbitrary;

fn main () {
	//let v = 15usize;
	for i in 1..100 {
		let v : i32 = arbitrary::arbitrary(0, 15);
		println!("{}: {}", i, v);
	}
	
	for i in 1..100 {
		let v : f32 = arbitrary::arbitrary(0, 15);
		println!("{}: {}", i, v);
	}
}