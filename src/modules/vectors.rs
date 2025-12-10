#[derive(Debug)]
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
	pub fn norm(&self) -> f64 {
		self.dot_with(&self).sqrt()
	}
	pub fn normalise(&self) -> NdVector<N> {
		let coords: [f64; N] = arrays::from_iter((0..N).map(|i| self.coords[i]/self.norm())).unwrap();
		NdVector {coords: coords}
	}
	pub fn project_to(&self, v: &NdVector<N>) -> NdVector<N> {
		let t: f64 = self.dot_with(v)/v.norm();
		let coords: [f64; N] = arrays::from_iter((0..N).map(|i| t*v.coords[i])).unwrap();
		NdVector {coords: coords}
	}
	pub fn project_and_sub(&self, v: &NdVector<N>) -> NdVector<N> {
		let coords: [f64; N] = arrays::from_iter((0..N).map(|i| self.coords[i] - self.project_to(v).coords[i])).unwrap();
		NdVector {coords: coords}	
	}
}

