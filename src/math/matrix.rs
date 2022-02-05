use std::fmt::Display;

pub struct Matrix<const ROW: usize, const COLUMN: usize> {
    pub row: usize,
    pub column: usize,
    content: [[f64; ROW]; COLUMN],
}

impl<const ROW: usize, const COLUMN: usize> Matrix<ROW, COLUMN> {

    pub const ROW: usize = ROW;
    pub const COLUMN: usize = COLUMN;

    pub fn new() -> Self {
        Self {
            row: ROW,
            column: COLUMN,
            content: [[0.0; ROW]; COLUMN],
        }
    }

    pub fn get_element<T: Into<usize> + Copy>(&self, i: T, j: T) -> Option<f64> {
        if i.into() > Self::ROW || j.into() > Self::COLUMN {
            Option::None
        } else {
            Option::Some(self.content[j.into()][i.into()])
        }
    }
    
    pub fn get_mut_element<T: Into<usize> + Copy>(&mut self, i: T, j: T) -> Option<&mut f64> {
        if i.into() > Self::ROW || j.into() > Self::COLUMN {
            Option::None
        } else {
            Option::Some(&mut self.content[j.into()][i.into()])
        }
    }
}

impl<const ROW: usize, const COLUMN: usize> Display for Matrix<{ROW}, {COLUMN}> {
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