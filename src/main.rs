mod modules;
use crate::modules::vectors::NdVector;
use crate::modules::matrices::Matrix;


fn main() {
	let mat: Matrix<3,2> = Matrix {entries: [[1.0, 4.2, 9.3],[-0.3, -0.6, 2.2]]};
	let v: NdVector<3> = NdVector{coords: [0.2, -0.9, 1.4]};
	println!("The product of this matrix by this vector is {:?}", mat.mul(&v).coords);   
	println!("The transpose of this matrix is {:?}", mat.transpose());
	println!("The second column of mat is {:?}", mat.recover_column(1));
	let a: NdVector<2> = NdVector{coords: [1.0, 1.0]};
	let b: NdVector<2> = NdVector{coords: [1.0, 0.0]};
	println!("The projection of 1,1 onto 1,0 is {:?}", a.project_to(&b).coords);
	println!("The projection of 1,0 onto 1,1 is {:?}", b.project_to(&a).coords);
	let m2: Matrix<2, 2> = Matrix::from_vectors(vec![a, b]);
	println!("Can we glue vectors together?: {:?}", m2);
	println!("The Gram-Schmidt algorithm applied to this matrix is {:?}", m2.gs());
	println!("The vectors a and b are linearly independent: {}", m2.lin_indep_rows());
	let c: NdVector<2> = NdVector{coords: [9.3, -0.2]};
	let m3: Matrix<2, 3> = Matrix::from_vectors(vec![a, b, c]);
	println!("The vectors, a, b, c linearly independent: {:?}", m3.lin_indep_rows());
}
