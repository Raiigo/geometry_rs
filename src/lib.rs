pub mod math;

#[cfg(test)]
mod tests {
    use crate::math::matrix::Matrix;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        let mat = Matrix::<1, 2>::new();
        let square_mat = Matrix::<2, 2>::identity();
        assert_eq!(square_mat.determinant(), 1.0);
    }
}
