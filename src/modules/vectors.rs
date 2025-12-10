#[derive(Debug)]
pub struct NdVector {
	pub coords: Vec<f64>,
}

impl NdVector {
	pub fn dot_with(&self, v: &NdVector) -> f64 {
		if self.coords.len() != v.coords.len() {
			panic!("These vectors do not live in the same vector space");
		}
		let mut t: f64 = 0.0;
		for i in 0..self.coords.len() {
			t += self.coords[i] * v.coords[i];
		}
		t
	}
	pub fn norm(&self) -> f64 {
		let t: f64 = self.dot_with(&self);
		t.sqrt()
	}
	pub fn scale_with(&self, v: &NdVector) -> f64 {
		let t: f64 = self.dot_with(v);
		t/self.norm()
	}
		
}
