pub mod math;

#[cfg(test)]
mod tests {
    use crate::math::matrix::Matrix;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let square_mat = Matrix::<2, 2>::identity();
        let identity3 = Matrix::<3, 3>::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        assert_eq!(square_mat.determinant(), 1.0);
        assert_eq!(identity3.determinant(), 1.0);
    }
}
