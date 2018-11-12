#![allow(dead_code, unused_imports, unused_variables)]
extern crate rand;

use self::rand::Rng;
use self::types::Bound;

mod types;

pub trait Arbitrary {
	fn generate(Bound) -> Self;
}

impl Arbitrary for i32 {
	fn generate(bounds: Bound) -> i32 {
		return rand::random();
	}
}

impl Arbitrary for f32 {
	fn generate(bounds: Bound) -> f32 {
		return rand::random();
	}
}

pub fn arbitrary<T: Arbitrary>(upper: i32, lower: i32) -> T {
	let bounds: Bound = Bound::new().upper_bound(upper).lower_bound(lower);
	return Arbitrary::generate(bounds);
}
