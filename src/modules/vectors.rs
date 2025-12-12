

#[derive(Debug, Copy, Clone)]
pub struct NdVector<const N: usize> {
	pub coords: [f64;N],
}

impl<const N: usize>  NdVector<N> {
	pub fn len(&self) -> usize {
		N
	}
	pub fn dot_with(&self, w: &NdVector<N>) -> f64 {
		// No need to compare lengths since both of them must have the same length.
		let mut t: f64 = 0.0;
		for i in 0..N {
			t += self.coords[i] * w.coords[i];
		}
		t
	}
	pub fn zero() -> NdVector<N> {
		NdVector {coords: [0.0; N]}
	}	
	pub fn norm(&self) -> f64 {
		self.dot_with(&self).sqrt()
	}
	pub fn normalise(&mut self) {
		let coords: [f64; N] = arrays::from_iter((0..N).map(|i| self.coords[i]/self.norm())).unwrap();
		self.coords = coords
	}
	pub fn project_to(&self, v: &NdVector<N>) -> NdVector<N> {
		let t: f64 = self.dot_with(v)/v.norm();
		let coords: [f64; N] = arrays::from_iter((0..N).map(|i| t*v.coords[i])).unwrap();
		NdVector {coords: coords}
	}
	pub fn project_and_sub(&mut self, v: &NdVector<N>) {
		let coords: [f64; N] = arrays::from_iter((0..N).map(|i| self.coords[i] - self.project_to(v).coords[i])).unwrap();
		self.coords = coords	
	}
}

