mod arbitrary;

fn main () {
	for i in 1..100 {
		let v : (String, i32, bool, Vec<i64>, f32, Vec<String>, char) = arbitrary::arbitrary(5);
		println!("{}: {:?}", i, v);
	}
}