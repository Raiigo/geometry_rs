use std::fmt::Display;
use std::ops::{Add, Mul};

pub struct VecD<const SIZE: usize> {
    pub size: usize,
    content: [f64; SIZE],
}

impl<const SIZE: usize> VecD<SIZE> {

    pub fn new() -> Self {
        Self {
            size: SIZE,
            content: [0.0; SIZE],
        }
    }

    pub fn get_element<T: Into<usize> + Copy>(&self, i: T) -> Option<f64> {
        if i.into() < SIZE {
            Option::Some(self.content[i.into()])
        } else {
            Option::None
        }
    }

    pub fn get_mut_element<T: Into<usize> + Copy>(&mut self, i: T) -> Option<&mut f64> {
        if i.into() < SIZE {
            Option::Some(&mut self.content[i.into()])
        } else {
            Option::None
        }
    }

    pub fn length(&self) -> f64 {
        self.content.into_iter().sum::<f64>().sqrt()
    }

    pub fn scalar(&self, vector: &Self) -> f64 {
        self.content.into_iter().enumerate().map(|(i, e)| {
            e * vector.content[i]
        }).collect::<Vec<f64>>().into_iter().sum::<f64>()
    }

    pub fn angle(&self, vector: &Self) -> f64 {
        (self.scalar(vector) / (self.length() * vector.length())).acos()
    }
}

impl<T: Into<f64> + Copy, const SIZE: usize> Mul<T> for &VecD<SIZE> {
    type Output = VecD::<SIZE>;

    fn mul(self, rhs: T) -> Self::Output {
        let content_vec = self.content.clone().into_iter().map(|e| {
            e * rhs.into()
        }).collect::<Vec<f64>>();
        let mut content = [0.0; SIZE];
        for (i, e) in content_vec.into_iter().enumerate() {
            content[i] = e;
        }
        
        VecD::<SIZE> {
            size: SIZE,
            content: content,
        }
    }
}

impl<const SIZE: usize> Add for &VecD<SIZE> {
    type Output = VecD::<SIZE>;

    fn add(self, rhs: Self) -> Self::Output {
        let content_vec: Vec<f64> = self.content.clone().into_iter().enumerate().map(|(i, e)| {
            e * rhs.content[i]
        }).collect::<Vec<f64>>();
        let mut content = [0.0; SIZE];
        for (i, e) in content_vec.into_iter().enumerate() {
            content[i] = e;
        }

        VecD::<SIZE> {
            size: SIZE,
            content: content,
        }
    }
}

impl<T: Into<f64> + Copy, const SIZE: usize> From<[T; SIZE]> for VecD<SIZE> {
    fn from(list: [T; SIZE]) -> Self {
        let mut content = [0.0; SIZE];
        for (i, e) in list.into_iter().enumerate() {
            content[i] = e.into();
        }
        
        Self {
            size: SIZE,
            content: content,
        }
    }
}

impl<const SIZE: usize> Display for VecD<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line: String = "(".into();
        for e in self.content {
            line.push_str(&format!("{}, ", e));
        }
        line.remove(line.len() - 1);
        line.remove(line.len() - 2);
        line.push(')');
        write!(f, "{}", line)
    }
}

impl<const SIZE: usize> Clone for VecD<SIZE> {
    fn clone(&self) -> Self {
        Self {
            size: SIZE,
            content: self.content.clone(),
        }
    }
}