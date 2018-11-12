pub struct Bound {
	upper_bound: i32,
	lower_bound: i32
}

impl Bound {
	pub fn new() -> Bound {
		Bound {upper_bound: 0, lower_bound: 0}
	}
	
	pub fn upper_bound(mut self, value: i32) -> Self {
		self.upper_bound = value;
		self
	}
	
	pub fn lower_bound(mut self, value: i32) -> Self {
		self.lower_bound = value;
		self
	}
}

