use crate::NdVector;

#[derive(Debug)]
pub struct Matrix<const N: usize, const M: usize> {
	pub entries: [[f64; N]; M],
}

impl<const N: usize, const M: usize> Matrix<N,M> {
	pub fn mul(&self, v: &NdVector<N>) -> NdVector<M> {
		let mut w: [f64; M]  = [0.0; M];
		for i in 0..M {
			let mut t: f64 = 0.0;
			for j in 0..N {
				t += &self.entries[i][j]*v.coords[j];
			}
			w[i] = t;
			}
		NdVector {coords: w}
	}
	pub fn transpose(&self) -> Matrix<M,N> {
		let mut tr_entries: [[f64;M]; N] = [[0.0; M]; N];
		for i in 0..M {
			for j in 0..N {
				tr_entries[j][i] = self.entries[i][j];
			}
		}
		Matrix {entries: tr_entries} 
	}	
	pub fn recover_column(&self, j: usize) -> NdVector<M> {
		let coords: [f64; M] = arrays::from_iter((0..M)
							.map(|i| self.entries[i][j]))
							.unwrap();
		NdVector {coords: coords}
	}
	pub fn recover_row(&self, i: usize) -> NdVector<N> {
		NdVector {coords: self.entries[i]}
	}
	pub fn square(&self) -> bool {
		N == M
	}
	pub fn from_vectors(lv: Vec<NdVector<N>>) -> Matrix<N, M> {
		if lv.len() != M {
			panic!("This list cannot be used to form a {}x{}-matrix", N, M);
		}
		let entries = arrays::from_iter((0..M)
						.map(|i| 
						{arrays::from_iter((0..N).map(|j| lv[i].coords[j])).unwrap()}))
										.unwrap();
		Matrix{entries: entries}
	}
	pub fn gs(&self) -> Matrix<N,M> {
                let mut holder: Vec<NdVector<N>> = Vec::new();
                for i in 0..M {
                        let mut row: NdVector<N> = self.recover_row(i);
                        for w in &holder {
                                row.project_and_sub(&w);
                        }
                        row.normalise();
                        holder.push(row);
                }
                Matrix::from_vectors(holder)
        }
}  
