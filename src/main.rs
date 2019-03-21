mod core;

fn tim_sort(v: &Vec<i32>) -> Vec<i32> {
	let mut vector = v.clone();
	vector.sort();
	vector
}

// Amazing O(1) Sorting
fn ao1_sort(v: &Vec<i32>) -> Vec<i32> {
	v.clone()
}

fn sort_check<F>(f: &'static F) -> impl Fn(&Vec<i32>) -> bool where 
	F: Fn(&Vec<i32>) -> Vec<i32> + Sized
{
	move |v| {
		let sorted = f(v);

		println!("Sorted vector: {:?}", sorted);
		
		for i in 1..sorted.len() {
			if sorted.get(i-1) > sorted.get(i) {return false}
		}
		true
	}
}

fn main () {
	let tim_sort_prop = sort_check(&tim_sort);
	let ao1_sort_prop = sort_check(&ao1_sort);
	 
	let test_params = core::TestParams{cases:100000, size: 5}; 
	
	println!("\n\n==== timsort test ====\n");
	core::run_test::<Vec<i32>>(&tim_sort_prop, &test_params);

	println!("\n\n==== ao1sort test ====\n");
	core::run_test::<Vec<i32>>(&ao1_sort_prop, &test_params);
}