use crate::NdVector;

#[derive(Debug)]
pub struct Matrix {
	pub source: usize,
	pub target: usize,
	pub entries: Vec<Vec<f64>>,
}

impl Matrix {
	pub fn from_string(f: &'static str) -> Matrix {
		let mut entries: Vec<Vec<f64>> = Vec::new();
		let rows: Vec<&str> = f.split(";").collect();
		for row in rows {
			let coords: Vec<&str> = row.split_whitespace().collect();
			let mut row_hold: Vec<f64> = Vec::new();
			for coord in coords {
				row_hold.push(coord.parse::<f64>().unwrap());
			}
			entries.push(row_hold);
		}
		let target: usize = entries.len();
		let source: usize = entries[0].len();
		Matrix{source: source, target: target, entries: entries}
	}
	pub fn mul(&self, v: &NdVector) -> NdVector {
		if self.source != v.coords.len() {
			panic!("This vector has the wrong dimension to be multiplied by this matrix");
		}
		else {  let mut w: Vec<f64> = Vec::new();
			for row in &self.entries {
				let mut t: f64 = 0.0;
				for i in 0..self.source {
					t += row[i]*v.coords[i];
				}
				w.push(t);
				}
		NdVector{coords: w}
		}
	}
	pub fn zero(source: usize, target: usize) -> Matrix {
		Matrix{source: source, target: target, entries: vec![vec![0.0; source];target]}
	}
	//pub fn transpose(&self) -> Matrix {
	//	let tr_entries: Vec<Vec<f64>> = (0..self.source).map( |s| { (0..self.target).map(|r| &self.entries[r][s]).collect()}).collect();
	//	Matrix{source:self.target, target: self.source, entries: tr_entries} 
	//}	
}  
