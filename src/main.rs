mod modules;
use crate::modules::vectors::NdVector;
use crate::modules::matrices::Matrix;



fn main() {
	let mat: Matrix = Matrix::from_string("1.0 4.2 9.3 ; -0.3 -0.6 2.2");
	let v: NdVector = NdVector{coords: vec![0.2, -0.9, 1.4]};
	println!("The product of this matrix by this vector is {:?}", mat.mul(&v).coords);   
	println!("The transpose of this matrix is {:?}", mat.transpose());
}
