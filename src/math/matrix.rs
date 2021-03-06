use crate::math::utils::combinatorics::permutations;
use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use super::{utils::combinatorics::parity, vector::VecD};

pub struct Matrix<const ROW: usize, const COLUMN: usize> {
    pub row: usize,
    pub column: usize,
    content: [[f64; COLUMN]; ROW],
}

impl<const SIZE: usize> Matrix<SIZE, SIZE> {
    pub const SIZE: usize = SIZE;
    pub fn determinant(&self) -> f64 {
        let permutations: Vec<Vec<usize>> = permutations(&(0..SIZE).collect::<Vec<usize>>());
        let mut sum = 0.0;
        for permutation in permutations {
            let p = parity(&permutation);
            let mut product = 1.0;
            for i in 0..SIZE {
                product = product * self.content[permutation[i]][i];
            }
            sum = sum + p as f64 * product;
        }
        sum
    }
}

impl<const ROW: usize, const COLUMN: usize> Matrix<ROW, COLUMN> {
    pub const ROW: usize = ROW;
    pub const COLUMN: usize = COLUMN;

    pub fn new() -> Self {
        Self {
            row: ROW,
            column: COLUMN,
            content: [[0.0; COLUMN]; ROW],
        }
    }

    pub fn full<T: Into<f64>>(value: T) -> Self {
        Self {
            row: ROW,
            column: COLUMN,
            content: [[value.into(); COLUMN]; ROW],
        }
    }

    pub fn identity() -> Self {
        let content_vec: Vec<Vec<f64>> = [[0.0; COLUMN]; ROW]
            .iter_mut()
            .enumerate()
            .map(|(i, l)| {
                l.into_iter()
                    .enumerate()
                    .map(|(j, e)| if i == j { 1.0 } else { *e })
                    .collect()
            })
            .collect();

        let mut content_array: [[f64; COLUMN]; ROW] = [[0.0; COLUMN]; ROW];

        for i in 0..ROW {
            for j in 0..COLUMN {
                content_array[i][j] = content_vec[i][j];
            }
        }

        Self {
            row: ROW,
            column: COLUMN,
            content: content_array,
        }
    }

    pub fn get_element<T: Into<usize> + Copy>(&self, i: T, j: T) -> Option<f64> {
        if i.into() > Self::ROW || j.into() > Self::COLUMN {
            Option::None
        } else {
            Option::Some(self.content[i.into()][j.into()])
        }
    }

    pub fn get_mut_element<T: Into<usize> + Copy>(&mut self, i: T, j: T) -> Option<&mut f64> {
        if i.into() > Self::ROW || j.into() > Self::COLUMN {
            Option::None
        } else {
            Option::Some(&mut self.content[i.into()][j.into()])
        }
    }

    pub fn transpose(&self) -> Matrix<COLUMN, ROW> {
        let mut new_content: [[f64; ROW]; COLUMN] = [[0.0; ROW]; COLUMN];
        for (i, l) in self.content.into_iter().enumerate() {
            for (j, e) in l.into_iter().enumerate() {
                new_content[j][i] = e;
            }
        }
        Matrix::<COLUMN, ROW> {
            row: ROW,
            column: COLUMN,
            content: new_content,
        }
    }
}

impl<const ROW: usize, const COLUMN: usize> Add for &Matrix<{ ROW }, { COLUMN }> {
    type Output = Option<Matrix<ROW, COLUMN>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.row == rhs.row && self.column == rhs.column {
            let new_content_vec: Vec<Vec<f64>> = self
                .content
                .into_iter()
                .enumerate()
                .map(|(i, l)| {
                    l.into_iter()
                        .enumerate()
                        .map(|(j, e)| e + rhs.content[i][j])
                        .collect()
                })
                .collect();
            let mut new_content_array: [[f64; COLUMN]; ROW] = [[0.0; COLUMN]; ROW];
            for i in 0..ROW {
                for j in 0..COLUMN {
                    new_content_array[i][j] = new_content_vec[i][j];
                }
            }
            Option::Some(Matrix {
                row: ROW,
                column: COLUMN,
                content: new_content_array,
            })
        } else {
            Option::None
        }
    }
}

impl<const ROW_1: usize, const COLUMN_1: usize, const ROW_2: usize, const COLUMN_2: usize>
    Mul<&Matrix<ROW_2, COLUMN_2>> for &Matrix<ROW_1, COLUMN_1>
{
    type Output = Option<Matrix<ROW_1, COLUMN_2>>;

    fn mul(self, rhs: &Matrix<ROW_2, COLUMN_2>) -> Self::Output {
        if COLUMN_1 == ROW_2 {
            let mut result = Matrix::<ROW_1, COLUMN_2>::new();
            for i_r in 0..result.row {
                for j_r in 0..result.column {
                    let mut sum = 0.0;
                    for c in 0..ROW_2 {
                        sum = sum + self.content[i_r][c] * rhs.content[c][j_r];
                    }
                    result.content[i_r][j_r] = sum;
                }
            }
            Option::Some(result)
        } else {
            Option::None
        }
    }
}

impl<T: Into<f64> + Copy, const ROW: usize, const COLUMN: usize> Mul<T> for &Matrix<ROW, COLUMN> {
    type Output = Matrix<ROW, COLUMN>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Matrix::<ROW, COLUMN>::new();
        for i in 0..ROW {
            for j in 0..COLUMN {
                result.content[i][j] = self.content[i][j] * rhs.into();
            }
        }
        result
    }
}

impl<T: Into<f64> + Copy, const ROW: usize, const COLUMN: usize> From<[[T; COLUMN]; ROW]> for Matrix<ROW, COLUMN> {
    fn from(list: [[T; COLUMN]; ROW]) -> Self {
        let mut content: [[f64; COLUMN]; ROW] = [[0.0; COLUMN]; ROW];
        for (i, l) in list.into_iter().enumerate() {
            for (j, e) in l.into_iter().enumerate() {
                content[i][j] = e.into();
            }
        }
        Self {
            row: ROW,
            column: COLUMN,
            content: content,
        }
    }
}

impl<const ROW: usize, const COLUMN: usize> Display for Matrix<{ ROW }, { COLUMN }> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines: String = String::from("");
        for i in 0..ROW {
            let mut line: String = "".into();
            for j in 0..COLUMN {
                let elem = match self.get_element(i, j) {
                    Some(e) => e,
                    None => 0.0,
                };
                line.push_str(&format!("{} ", elem));
            }
            line.push_str("\n");
            lines.push_str(&line);
        }
        write!(f, "{}", lines)
    }
}

impl<const SIZE: usize> From<VecD::<SIZE>> for Matrix<SIZE, 1> {
    fn from(vector: VecD::<SIZE>) -> Self {
        let mut matrix = Matrix::<SIZE, 1>::new();
        for i in 0..SIZE {
            *matrix.get_mut_element(i, 0).unwrap() = vector.get_element(i).unwrap();
        }
        matrix
    }
}

pub struct MatrixIterator<const ROW: usize, const COLUMN: usize> {
    matrix: Matrix<ROW, COLUMN>,
    counter: usize,
}

impl<const ROW: usize, const COLUMN: usize> From<Matrix<ROW, COLUMN>> for MatrixIterator<ROW, COLUMN> {
    fn from(matrix: Matrix<ROW, COLUMN>) -> Self {
        Self {
            matrix: matrix,
            counter: 0,
        }
    }
}

impl<const ROW: usize, const COLUMN: usize> Iterator for MatrixIterator<ROW, COLUMN> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let result;
        if self.counter < ROW * COLUMN {
            let i = self.counter / COLUMN;
            let j = self.counter % COLUMN;
            result = self.matrix.get_element(i, j);
        } else {
            result = None;
        }
        self.counter += 1;
        return result;
    }
}
// 3x3 : 0 -> (0, 0) 1 -> (0, 1) 2 -> (0, 2)
//       3 -> (1, 0) 4 -> (1, 1) 3 -> (1, 2)