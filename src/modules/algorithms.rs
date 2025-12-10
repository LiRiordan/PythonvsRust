use crate::NdVector;
use crate::Matrix;

pub enum Algorithm {
	Gram-Schmidt,
	RREF,
	SVD,
}

pub struct Gram-Schmidt {
}

pub struct RREF {
}

pub struct SVD {
}

impl<const N: usize, const M: usize> Matrix<N,M> {
	pub fn gs(&self) -> Matrix {
		if !self.square {
			panic!("This matrix is not square");
		}
		let mut gs_entries: Vec<Vec<f64>> = Vec::new();
		for i in 0..self.target {
			let mut v: Vec<f64> = self.recover_row(i).coords.clone();
			for w in gs_entries {
					let v: Vec<f64> 
	}
}

